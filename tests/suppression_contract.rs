use mailchannels_email_api::{apis::suppression_api::*, models::SuppressionListInput};
mod common;
use common::fixture;
#[tokio::test]
async fn create_suppressions_preserves_scope_types_and_nullable_notes() {
    let expected = serde_json::json!({"add_to_sub_accounts":true,"suppression_entries":[{"recipient":"one+tag@example.invalid","notes":null,"suppression_types":["transactional","non-transactional"]},{"recipient":"two@example.invalid","notes":"fixture note"}]});
    let input: SuppressionListInput = serde_json::from_value(expected.clone()).unwrap();
    let (config, server) = fixture(201, "");
    assert!(matches!(
        create_suppressions(&config, "fixture-key", input)
            .await
            .unwrap()
            .entity,
        Some(CreateSuppressionsSuccess::Status201())
    ));
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /suppression-list HTTP/1.1"));
    let body: serde_json::Value =
        serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(body, expected);
}
#[tokio::test]
async fn suppression_listing_preserves_filters_and_nullable_fields() {
    let (config, server) = fixture(
        200,
        r#"{"suppression_list":[{"recipient":"one+tag@example.invalid","sender":null,"notes":null,"source":"spam_complaint","created_at":"2026-10-01T00:00:00Z","suppression_types":["non-transactional"]}]}"#,
    );
    match list_suppressions(
        &config,
        "fixture-key",
        Some("one+tag@example.invalid"),
        Some("spam_complaint"),
        Some("2026-10-02"),
        Some("2026-10-01"),
        Some(5),
        Some(10),
    )
    .await
    .unwrap()
    .entity
    .unwrap()
    {
        ListSuppressionsSuccess::Status200(response) => {
            assert_eq!(response.suppression_list.len(), 1);
            let item = &response.suppression_list[0];
            assert_eq!(item.recipient, "one+tag@example.invalid");
            assert_eq!(item.sender, Some(None));
            assert_eq!(item.notes, Some(None));
            let encoded = serde_json::to_value(item).unwrap();
            assert_eq!(encoded["source"], "spam_complaint");
            assert_eq!(
                encoded["suppression_types"],
                serde_json::json!(["non-transactional"])
            );
        }
        _ => panic!("Wrong suppression response"),
    }
    let request = server.join().unwrap();
    let target = request.split_whitespace().nth(1).unwrap();
    let url = url::Url::parse(&format!("http://fixture{target}")).unwrap();
    assert_eq!(url.path(), "/suppression-list");
    let params: Vec<_> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    assert_eq!(
        params,
        vec![
            ("recipient".into(), "one+tag@example.invalid".into()),
            ("source".into(), "spam_complaint".into()),
            ("created_before".into(), "2026-10-02".into()),
            ("created_after".into(), "2026-10-01".into()),
            ("limit".into(), "5".into()),
            ("offset".into(), "10".into())
        ]
    );
}
#[tokio::test]
async fn deletion_encodes_recipient_and_preserves_default_source() {
    for source in [None, Some("all")] {
        let (config, server) = fixture(204, "");
        assert!(matches!(
            delete_suppression(&config, "one+tag@example.invalid", "fixture-key", source)
                .await
                .unwrap()
                .entity,
            Some(DeleteSuppressionSuccess::Status204())
        ));
        let request = server.join().unwrap();
        let suffix = if source.is_some() { "?source=all" } else { "" };
        assert!(request.starts_with(&format!(
            "DELETE /suppression-list/recipients/one%2Btag%40example.invalid{suffix} HTTP/1.1"
        )));
        assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
    }
}
