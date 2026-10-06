use mailchannels_email_api::{
    apis::{
        configuration::Configuration,
        send_api::{queue_email, send_email, QueueEmailSuccess, SendEmailError, SendEmailSuccess},
        Error,
    },
    models::MailSendBody,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

mod common;
use common::fixture;
fn message() -> MailSendBody {
    serde_json::from_str(r#"{"from":{"email":"sender@example.invalid"},"personalizations":[{"to":[{"email":"recipient@example.invalid"}]}],"subject":"fixture","content":[{"type":"text/plain","value":"fixture"}]}"#).unwrap()
}
#[tokio::test]
async fn accepted_send_retains_request_id_and_partial_results() {
    let (config, handle) = fixture(
        202,
        r#"{"request_id":"fixture-request","results":[{"index":0,"message_id":"fixture-message","status":"sent"},{"index":1,"status":"failed","reason":"fixture rejection"}]}"#,
    );
    let response = send_email(&config, "fixture-key", message(), None)
        .await
        .unwrap();
    match response.entity.unwrap() {
        SendEmailSuccess::Status202(result) => {
            assert_eq!(result.request_id.as_deref(), Some("fixture-request"));
            let results = result.results.unwrap();
            assert_eq!(results.len(), 2);
            assert_eq!(results[1].reason.as_deref(), Some("fixture rejection"));
        }
        other => panic!("Wrong response model: {other:?}"),
    }
    let request = handle.join().unwrap();
    assert!(request.starts_with("POST /send HTTP/1.1"));
    assert!(request.to_lowercase().contains("x-api-key: fixture-key"));
}
#[tokio::test]
async fn dry_run_uses_200_model_and_query() {
    let (config, handle) = fixture(200, r#"{"data":["rendered fixture"]}"#);
    let response = send_email(&config, "fixture-key", message(), Some(true))
        .await
        .unwrap();
    match response.entity.unwrap() {
        SendEmailSuccess::Status200(result) => {
            assert_eq!(result.data.unwrap(), vec!["rendered fixture"])
        }
        other => panic!("Wrong response model: {other:?}"),
    }
    assert!(handle
        .join()
        .unwrap()
        .starts_with("POST /send?dry-run=true HTTP/1.1"));
}
#[tokio::test]
async fn empty_accepted_response_has_no_invented_entity() {
    let (config, handle) = fixture(202, "");
    let response = send_email(&config, "fixture-key", message(), None)
        .await
        .unwrap();
    assert_eq!(response.status.as_u16(), 202);
    assert!(response.entity.is_none());
    handle.join().unwrap();
}
#[tokio::test]
async fn malformed_success_is_an_error() {
    let (config, handle) = fixture(202, "not json");
    assert!(matches!(
        send_email(&config, "fixture-key", message(), None).await,
        Err(Error::Serde(_))
    ));
    handle.join().unwrap();
}
#[tokio::test]
async fn error_variant_matches_status_not_shape() {
    let (config, handle) = fixture(403, r#"{"errors":["fixture denied"]}"#);
    match send_email(&config, "fixture-key", message(), None)
        .await
        .unwrap_err()
    {
        Error::ResponseError(response) => assert!(matches!(
            response.entity,
            Some(SendEmailError::Status403(_))
        )),
        other => panic!("Wrong error: {other:?}"),
    }
    handle.join().unwrap();
}
#[tokio::test]
async fn redirect_is_not_success() {
    let (config, handle) = fixture(302, "");
    assert!(matches!(
        send_email(&config, "fixture-key", message(), None).await,
        Err(Error::ResponseError(_))
    ));
    handle.join().unwrap();
}
#[tokio::test]
async fn async_send_retains_receipt() {
    let (config, handle) = fixture(
        202,
        r#"{"request_id":"fixture-async","queued_at":"2026-10-06T00:00:00Z"}"#,
    );
    let response = queue_email(&config, "fixture-key", message())
        .await
        .unwrap();
    match response.entity.unwrap() {
        QueueEmailSuccess::Status202(result) => assert_eq!(result.request_id, "fixture-async"),
        other => panic!("Wrong response: {other:?}"),
    }
    assert!(handle
        .join()
        .unwrap()
        .starts_with("POST /send-async HTTP/1.1"));
}

#[tokio::test]
async fn production_client_refuses_plain_http_before_connecting() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut config = Configuration::new();
    config.base_path = format!("http://{}", listener.local_addr().unwrap());
    assert!(matches!(
        send_email(&config, "fixture-key", message(), None).await,
        Err(Error::Reqwest(_))
    ));
    assert!(matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
}

#[tokio::test]
async fn transport_does_not_follow_redirect_or_replay_503() {
    use mailchannels_email_api::apis::configuration::transport_builder;
    for status in [302, 307, 503] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .unwrap();
            let mut buffer = [0; 4096];
            assert!(stream.read(&mut buffer).unwrap() > 0);
            write!(stream, "HTTP/1.1 {status} Fixture\r\nLocation: http://{address}/redirected\r\nRetry-After: 0\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
            drop(stream);
            listener.set_nonblocking(true).unwrap();
            let end = std::time::Instant::now() + std::time::Duration::from_millis(200);
            while std::time::Instant::now() < end {
                match listener.accept() {
                    Ok(_) => panic!("Unexpected redirected or retried request"),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(5))
                    }
                    Err(e) => panic!("{e}"),
                }
            }
        });
        let client = transport_builder()
            .https_only(false)
            .no_proxy()
            .build()
            .unwrap();
        let response = client
            .post(format!("http://{address}/send"))
            .body("fixture")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), status);
        handle.join().unwrap();
    }
}

#[tokio::test]
async fn configured_timeout_bounds_stalled_response() {
    use mailchannels_email_api::apis::configuration::transport_builder;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (_stream, _) = listener.accept().unwrap();
        thread::sleep(std::time::Duration::from_millis(250));
    });
    let client = transport_builder()
        .https_only(false)
        .no_proxy()
        .timeout(std::time::Duration::from_millis(50))
        .build()
        .unwrap();
    let started = std::time::Instant::now();
    let error = client
        .get(format!("http://{address}/fixture"))
        .send()
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert!(started.elapsed() < std::time::Duration::from_secs(2));
    handle.join().unwrap();
}

#[tokio::test]
async fn provider_error_debug_redacts_body_and_typed_entity() {
    let (config, handle) = fixture(
        403,
        r#"{"errors":["fixture-secret-key and private message"]}"#,
    );
    let error = send_email(&config, "fixture-key", message(), None)
        .await
        .unwrap_err();
    let printed = format!("{error} {error:?}");
    assert!(!printed.contains("fixture-secret-key"));
    assert!(!printed.contains("private message"));
    assert!(printed.contains("403"));
    assert!(std::error::Error::source(&error).is_none());
    match error {
        Error::ResponseError(response) => {
            assert!(!format!("{response:?}").contains("fixture-secret-key"));
            assert!(response.content.contains("fixture-secret-key")); // explicit access remains
        }
        _ => panic!("Expected provider error"),
    }
    handle.join().unwrap();
}

#[tokio::test]
async fn transport_debug_does_not_expose_url_secrets() {
    let error = Configuration::new()
        .client
        .get("http://127.0.0.1/?key=fixture-url-secret")
        .send()
        .await
        .unwrap_err();
    let error: Error<()> = error.into();
    assert!(!format!("{error} {error:?}").contains("fixture-url-secret"));
    assert!(std::error::Error::source(&error).is_none());
}
