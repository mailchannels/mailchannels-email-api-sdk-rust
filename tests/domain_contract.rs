use mailchannels_email_api::apis::{custom_tracking_api::*, dkim_api::*};
mod common;
use common::fixture;
fn body(request: &str) -> serde_json::Value {
    serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap()
}
const KEY: &str = r#"{"domain":"example.invalid","selector":"fixture","public_key":"fixture-public-key","status":"active","algorithm":"rsa","created_at":null,"key_length":2048,"dkim_dns_records":[{"name":"fixture._domainkey.example.invalid","type":"TXT","value":"fixture-dns"}]}"#;
const DOMAIN: &str = r#"{"name":"fixture","hostname":"click.example.invalid","scope":"click","status":"active","created_at":"2026-10-01T00:00:00Z"}"#;
const DNS: &str = r#"{"instructions":"fixture DNS required","token":"fixture-token","txt_record_name":"_mailchannels-verify.click.example.invalid","txt_record_value":"fixture-token"}"#;
#[tokio::test]
async fn domain_check_preserves_envelope_domain_and_failed_verdict() {
    let expected = serde_json::json!({"domain":"example.invalid","envelope_from_domain":"bounce.example.invalid","sender_id":"fixture","dkim_settings":[{"dkim_domain":"example.invalid","dkim_selector":"fixture"}]});
    let (config, server) = fixture(
        200,
        r#"{"check_results":{"dkim":[{"verdict":"failed","reason":"fixture mismatch"}]},"references":["https://example.invalid/help"]}"#,
    );
    let response = check_domain(
        &config,
        "fixture-key",
        serde_json::from_value(expected.clone()).unwrap(),
    )
    .await
    .unwrap();
    let result = serde_json::to_value(response.entity.unwrap()).unwrap();
    assert_eq!(result["check_results"]["dkim"][0]["verdict"], "failed");
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /check-domain HTTP/1.1"));
    assert_eq!(body(&request), expected);
}
#[tokio::test]
async fn create_dkim_preserves_algorithm_and_dns_record() {
    let expected = serde_json::json!({"selector":"fixture","algorithm":"rsa","key_length":2048});
    let (config, server) = fixture(201, KEY);
    let response = create_dkim_key(
        &config,
        "example.invalid",
        "fixture-key",
        serde_json::from_value(expected.clone()).unwrap(),
    )
    .await
    .unwrap();
    let result = serde_json::to_value(response.entity.unwrap()).unwrap();
    assert_eq!(result["selector"], "fixture");
    assert_eq!(result["dkim_dns_records"][0]["type"], "TXT");
    assert!(result["created_at"].is_null());
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /domains/example.invalid/dkim-keys HTTP/1.1"));
    assert_eq!(body(&request), expected);
}
#[tokio::test]
async fn list_dkim_preserves_filters_and_empty_result() {
    let (config, server) = fixture(200, r#"{"keys":[]}"#);
    assert!(
        matches!(list_dkim_keys(&config,"example.invalid","fixture-key",Some("fixture"),Some("active"),Some(10),Some(5),Some(true)).await.unwrap().entity,Some(ListDkimKeysSuccess::Status200(result)) if result.keys.is_empty())
    );
    let request = server.join().unwrap();
    assert!(request.starts_with("GET /domains/example.invalid/dkim-keys?selector=fixture&status=active&offset=10&limit=5&include_dns_record=true HTTP/1.1"));
}
#[tokio::test]
async fn rotate_dkim_keeps_old_and_new_selectors_distinct() {
    let (config, server) = fixture(
        201,
        r#"{"new_key":{"domain":"example.invalid","selector":"new","public_key":"new-public","status":"active","algorithm":"rsa"},"rotated_key":{"domain":"example.invalid","selector":"old","public_key":"old-public","status":"rotated","algorithm":"rsa","retiresAt":null,"gracePeriodExpiresAt":null}}"#,
    );
    let response = rotate_dkim_key(
        &config,
        "example.invalid",
        "old",
        "fixture-key",
        serde_json::from_value(serde_json::json!({"new_key":{"selector":"new"}})).unwrap(),
    )
    .await
    .unwrap();
    let result = serde_json::to_value(response.entity.unwrap()).unwrap();
    assert_eq!(result["new_key"]["selector"], "new");
    assert_eq!(result["rotated_key"]["selector"], "old");
    assert_eq!(result["rotated_key"]["status"], "rotated");
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /domains/example.invalid/dkim-keys/old/rotate HTTP/1.1"));
    assert_eq!(
        body(&request),
        serde_json::json!({"new_key":{"selector":"new"}})
    );
}
#[tokio::test]
async fn update_dkim_sends_revocation_and_accepts_empty_204() {
    let (config, server) = fixture(204, "");
    assert!(matches!(
        update_dkim_key(
            &config,
            "example.invalid",
            "fixture",
            "fixture-key",
            serde_json::from_value(serde_json::json!({"status":"revoked"})).unwrap()
        )
        .await
        .unwrap()
        .entity,
        Some(UpdateDkimKeySuccess::Status204())
    ));
    let request = server.join().unwrap();
    assert!(request.starts_with("PATCH /domains/example.invalid/dkim-keys/fixture HTTP/1.1"));
    assert_eq!(body(&request), serde_json::json!({"status":"revoked"}));
}
#[tokio::test]
async fn tracking_create_distinguishes_active_and_dns_pending() {
    for (status, response_body) in [(201, DOMAIN), (202, DNS)] {
        let (config, server) = fixture(status, response_body);
        let expected = serde_json::json!({"name":"fixture","hostname":"click.example.invalid","scope":"click"});
        match create_custom_tracking_domain(
            &config,
            "fixture-key",
            serde_json::from_value(expected.clone()).unwrap(),
        )
        .await
        .unwrap()
        .entity
        .unwrap()
        {
            CreateCustomTrackingDomainSuccess::Status201(domain) => {
                assert_eq!(status, 201);
                assert_eq!(domain.hostname, "click.example.invalid");
            }
            CreateCustomTrackingDomainSuccess::Status202(dns) => {
                assert_eq!(status, 202);
                assert_eq!(dns.token.as_deref(), Some("fixture-token"));
            }
            _ => panic!("Wrong tracking result"),
        }
        let request = server.join().unwrap();
        assert!(request.starts_with("POST /custom-tracking-domains HTTP/1.1"));
        assert_eq!(body(&request), expected);
    }
}
#[tokio::test]
async fn tracking_list_preserves_filters_and_total() {
    let (config, server) = fixture(200, r#"{"custom_tracking_domains":[],"total":0}"#);
    assert!(
        matches!(list_custom_tracking_domains(&config,"fixture-key",Some("fixture"),Some("active"),Some("click"),Some(5),Some(10)).await.unwrap().entity,Some(ListCustomTrackingDomainsSuccess::Status200(result)) if result.total==0 && result.custom_tracking_domains.is_empty())
    );
    assert!(server.join().unwrap().starts_with("GET /custom-tracking-domains?name=fixture&status=active&scope=click&limit=5&offset=10 HTTP/1.1"));
}
#[tokio::test]
async fn tracking_update_distinguishes_success_and_dns_pending() {
    for (status, response_body) in [(200, DOMAIN), (202, DNS)] {
        let (config, server) = fixture(status, response_body);
        let expected = serde_json::json!({"status":"active"});
        match update_custom_tracking_domain(
            &config,
            "click.example.invalid",
            "click",
            "fixture-key",
            serde_json::from_value(expected.clone()).unwrap(),
        )
        .await
        .unwrap()
        .entity
        .unwrap()
        {
            UpdateCustomTrackingDomainSuccess::Status200(domain) => {
                assert_eq!(status, 200);
                assert_eq!(domain.hostname, "click.example.invalid");
            }
            UpdateCustomTrackingDomainSuccess::Status202(dns) => {
                assert_eq!(status, 202);
                assert_eq!(dns.instructions.as_deref(), Some("fixture DNS required"));
            }
            _ => panic!("Wrong tracking result"),
        }
        let request = server.join().unwrap();
        assert!(request
            .starts_with("PATCH /custom-tracking-domains/click.example.invalid/click HTTP/1.1"));
        assert_eq!(body(&request), expected);
    }
}
#[tokio::test]
async fn tracking_delete_addresses_hostname_and_scope() {
    let (config, server) = fixture(204, "");
    assert!(matches!(
        delete_custom_tracking_domain(&config, "click.example.invalid", "click", "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(DeleteCustomTrackingDomainSuccess::Status204())
    ));
    let request = server.join().unwrap();
    assert!(
        request.starts_with("DELETE /custom-tracking-domains/click.example.invalid/click HTTP/1.1")
    );
    assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
}
