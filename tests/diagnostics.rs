use mailchannels_email_api::{
    apis::{
        configuration::{ApiKey as ConfigKey, Configuration},
        send_api::SendEmailSuccess,
    },
    models::*,
};
#[test]
fn credential_models_do_not_print_secrets() {
    let key: ApiKey = serde_json::from_str(r#"{"id":1,"key":"fixture-secret"}"#).unwrap();
    let smtp: SmtpPassword =
        serde_json::from_str(r#"{"id":1,"smtp_password":"fixture-secret"}"#).unwrap();
    assert!(!format!("{key:?} {smtp:#?}").contains("fixture-secret"));
    assert_eq!(key.key.as_deref(), Some("fixture-secret"));
    assert_eq!(
        serde_json::to_value(smtp).unwrap()["smtp_password"],
        "fixture-secret"
    );
}
#[test]
fn sending_model_debug_omits_content_and_dkim_material() {
    let message:MailSendBody=serde_json::from_str(r#"{"from":{"email":"fixture-private@example.invalid"},"personalizations":[{"to":[{"email":"recipient@example.invalid"}]}],"subject":"fixture-private-subject","dkim_private_key":"fixture-private-key","content":[{"type":"text/plain","value":"fixture-private-body"}]}"#).unwrap();
    let debug = format!("{message:#?}");
    assert!(!debug.contains("fixture-private"));
    let wire = serde_json::to_value(message).unwrap();
    assert_eq!(wire["content"][0]["value"], "fixture-private-body");
}
#[test]
fn configuration_debug_omits_caller_supplied_urls_and_headers() {
    let mut config = Configuration::new();
    config.base_path = "https://example.invalid/?key=fixture-secret".into();
    config.user_agent = Some("fixture-secret".into());
    config.basic_auth = Some(("fixture-secret".into(), Some("fixture-secret".into())));
    config.api_key = Some(ConfigKey {
        prefix: Some("fixture-secret".into()),
        key: "fixture-secret".into(),
    });
    assert!(!format!("{config:#?}").contains("fixture-secret"));
    assert!(!format!("{:?}", config.api_key.unwrap()).contains("fixture-secret"));
}
#[test]
fn unknown_response_variant_debug_omits_json() {
    let response = SendEmailSuccess::UnknownValue(serde_json::json!({"echo":"fixture-secret"}));
    assert!(!format!("{response:?}").contains("fixture-secret"));
    assert_eq!(
        serde_json::to_value(response).unwrap()["echo"],
        "fixture-secret"
    );
}
