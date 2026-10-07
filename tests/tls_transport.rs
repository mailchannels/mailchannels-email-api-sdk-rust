use mailchannels_email_api::{
    apis::{
        configuration::{transport_builder, Configuration},
        send_api::{send_email, SendEmailSuccess},
        Error,
    },
    models::MailSendBody,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::Arc,
    thread,
    time::Duration,
};

async fn probe(label: &str, trust: bool, accepted: bool) {
    let root = std::path::Path::new("tests/tls-fixtures");
    let cert = rustls::pki_types::CertificateDer::from(
        std::fs::read(root.join(format!("{label}.der"))).unwrap(),
    );
    let key = rustls::pki_types::PrivatePkcs8KeyDer::from(
        std::fs::read(root.join(format!("{label}-key.der"))).unwrap(),
    );
    let server_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key.into())
        .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let server = thread::spawn(move || {
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        let tcp = loop {
            match listener.accept() {
                Ok((tcp, _)) => break tcp,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        && std::time::Instant::now() < deadline =>
                {
                    thread::sleep(Duration::from_millis(5))
                }
                Err(e) => panic!("TLS fixture accept failed: {e}"),
            }
        };
        // Use blocking I/O with deadlines regardless of listener inheritance.
        tcp.set_nonblocking(false).unwrap();
        tcp.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        tcp.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        let connection = rustls::ServerConnection::new(Arc::new(server_config)).unwrap();
        let mut stream = rustls::StreamOwned::new(connection, tcp);
        let mut request = Vec::new();
        loop {
            let mut buf = [0; 4096];
            match stream.read(&mut buf) {
                Ok(0) | Err(_) => return request,
                Ok(n) => request.extend_from_slice(&buf[..n]),
            }
            if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&request[..end]).to_lowercase();
                let length: usize = headers
                    .lines()
                    .find_map(|line| line.strip_prefix("content-length: "))
                    .unwrap()
                    .parse()
                    .unwrap();
                if request.len() >= end + 4 + length {
                    break;
                }
            }
        }
        let body = r#"{"request_id":"tls-fixture","results":[]}"#;
        write!(stream,"HTTP/1.1 202 Accepted\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).unwrap();
        stream.flush().unwrap();
        request
    });
    let mut builder = transport_builder()
        .no_proxy()
        .resolve("fixture.test", address)
        .timeout(Duration::from_secs(3));
    if trust {
        builder = builder.add_root_certificate(
            reqwest::Certificate::from_der(&std::fs::read(root.join("ca.der")).unwrap()).unwrap(),
        );
    }
    let mut config = Configuration::new();
    config.client = builder.build().unwrap();
    config.base_path = format!("https://fixture.test:{}", address.port());
    let message: MailSendBody=serde_json::from_str(r#"{"from":{"email":"sender@example.invalid"},"personalizations":[{"to":[{"email":"recipient@example.invalid"}]}],"subject":"fixture","content":[{"type":"text/plain","value":"fixture"}]}"#).unwrap();
    let result = send_email(&config, "fixture-tls-secret", message, None).await;
    let received = server.join().unwrap();
    if accepted {
        assert!(
            matches!(result.unwrap().entity,Some(SendEmailSuccess::Status202(receipt)) if receipt.request_id.as_deref()==Some("tls-fixture"))
        );
        assert!(String::from_utf8(received)
            .unwrap()
            .contains("fixture-tls-secret"));
    } else {
        assert!(
            matches!(result, Err(Error::Reqwest(_))),
            "TLS verification must reject before HTTP"
        );
        assert!(
            received.is_empty(),
            "No HTTP credentials/body may reach rejected peer"
        );
    }
}
#[tokio::test]
async fn trusted_matching_certificate_sends() {
    probe("valid", true, true).await;
}
#[tokio::test]
async fn untrusted_certificate_rejected() {
    probe("valid", false, false).await;
}
#[tokio::test]
async fn wrong_hostname_rejected() {
    probe("wrong-host", true, false).await;
}
#[tokio::test]
async fn expired_certificate_rejected() {
    probe("expired", true, false).await;
}
