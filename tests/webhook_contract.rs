use mailchannels_email_api::{apis::webhooks_api::*, models::WebhookValidationRequestBody};
mod common;
use common::fixture;
fn query(request: &str) -> Vec<(String, String)> {
    let target = request.split_whitespace().nth(1).unwrap();
    url::Url::parse(&format!("http://fixture{target}"))
        .unwrap()
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}
#[tokio::test]
async fn enrollment_encodes_endpoint_as_one_query_value() {
    let endpoint = "https://example.invalid/hook?a=1&token=fixture+value#frag";
    let (config, server) = fixture(201, "");
    assert!(matches!(
        create_webhook(&config, endpoint, "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(CreateWebhookSuccess::Status201())
    ));
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /webhook?"));
    assert_eq!(query(&request), vec![("endpoint".into(), endpoint.into())]);
    assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
}
#[tokio::test]
async fn webhook_listing_and_deletion_preserve_contract() {
    let (config, server) = fixture(
        200,
        r#"[{"webhook":"https://example.invalid/a"},{"webhook":"https://example.invalid/b"}]"#,
    );
    assert!(
        matches!(list_webhooks(&config,"fixture-key").await.unwrap().entity,Some(ListWebhooksSuccess::Status200(items)) if items.len()==2 && items[1].webhook=="https://example.invalid/b")
    );
    assert!(server.join().unwrap().starts_with("GET /webhook HTTP/1.1"));
    let (config, server) = fixture(204, "");
    assert!(matches!(
        delete_webhooks(&config, "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(DeleteWebhooksSuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("DELETE /webhook HTTP/1.1"));
}
#[tokio::test]
async fn signing_key_uses_public_query_without_api_key() {
    let (config, server) = fixture(200, r#"{"id":"fixture+key","key":"fixture-public-key"}"#);
    assert!(
        matches!(get_webhook_signing_key(&config,"fixture+key").await.unwrap().entity,Some(GetWebhookSigningKeySuccess::Status200(key)) if key.id=="fixture+key" && key.key=="fixture-public-key")
    );
    let request = server.join().unwrap();
    assert_eq!(query(&request), vec![("id".into(), "fixture+key".into())]);
    assert!(!request.to_lowercase().contains("x-api-key:"));
}
#[tokio::test]
async fn batch_filters_and_nullable_response_are_preserved() {
    let (config, server) = fixture(
        200,
        r#"{"webhook_batches":[{"batch_id":4294967296,"customer_handle":"fixture","webhook":"https://example.invalid/hook","status":"no_response","status_code":null,"created_at":"2026-10-01T00:00:00Z","event_count":2}]}"#,
    );
    let response = list_webhook_batches(
        &config,
        "fixture-key",
        Some("2026-10-01"),
        Some("2026-10-02"),
        Some(vec!["no_response".into(), "5xx".into()]),
        Some("https://example.invalid/hook?a=1&b=2"),
        Some(10),
        Some(20),
    )
    .await
    .unwrap();
    match response.entity.unwrap() {
        ListWebhookBatchesSuccess::Status200(result) => {
            assert_eq!(result.webhook_batches[0].batch_id, 4294967296);
            assert_eq!(result.webhook_batches[0].status_code, Some(None));
            assert_eq!(result.webhook_batches[0].event_count, 2);
        }
        _ => panic!("Wrong batch model"),
    }
    let request = server.join().unwrap();
    let params = query(&request);
    assert_eq!(
        params,
        vec![
            ("created_after".into(), "2026-10-01".into()),
            ("created_before".into(), "2026-10-02".into()),
            ("statuses".into(), "no_response,5xx".into()),
            (
                "webhook".into(),
                "https://example.invalid/hook?a=1&b=2".into()
            ),
            ("limit".into(), "10".into()),
            ("offset".into(), "20".into())
        ]
    );
}
#[tokio::test]
async fn resend_batch_preserves_nullable_receipt() {
    let (config, server) = fixture(
        200,
        r#"{"batch_id":4294967296,"customer_handle":"fixture","webhook":"https://example.invalid/hook","created_at":"2026-10-01T00:00:00Z","event_count":2,"status_code":null,"duration_in_ms":null}"#,
    );
    match resend_webhook_batch(&config, 4294967296, "fixture-key")
        .await
        .unwrap()
        .entity
        .unwrap()
    {
        ResendWebhookBatchSuccess::Status200(receipt) => {
            assert_eq!(receipt.batch_id, 4294967296);
            assert_eq!(receipt.status_code, Some(None));
            assert_eq!(receipt.duration_in_ms, Some(None));
        }
        _ => panic!("Wrong resend model"),
    }
    assert!(server
        .join()
        .unwrap()
        .starts_with("POST /webhook-batch/4294967296/resend HTTP/1.1"));
}
#[tokio::test]
async fn validation_preserves_request_identifier_and_failure_flag() {
    let (config, server) = fixture(200, r#"{"all_passed":false,"results":[]}"#);
    let mut body = WebhookValidationRequestBody::new();
    body.request_id = Some("fixture-request".into());
    assert!(
        matches!(validate_webhook(&config,"fixture-key",Some(body)).await.unwrap().entity,Some(ValidateWebhookSuccess::Status200(result)) if !result.all_passed)
    );
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /webhook/validate HTTP/1.1"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request.split_once("\r\n\r\n").unwrap().1)
            .unwrap(),
        serde_json::json!({"request_id":"fixture-request"})
    );
}
