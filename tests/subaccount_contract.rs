use mailchannels_email_api::{
    apis::{sub_accounts_api::*, Error},
    models::LimitInput,
};
mod common;
use common::fixture;

#[tokio::test]
async fn inherited_limit_and_encoded_handle() {
    let (config, server) = fixture(200, r#"{"sends":-1}"#);
    let response = get_subaccount_limit(&config, "team/a b+?#", "fixture-key")
        .await
        .unwrap();
    assert!(
        matches!(response.entity,Some(GetSubaccountLimitSuccess::Status200(limit)) if limit.sends == -1)
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /sub-account/team%2Fa%20b%2B%3F%23/limit HTTP/1.1"));
}
#[tokio::test]
async fn zero_limit_is_not_null_or_omitted() {
    let (config, server) = fixture(200, r#"{"limit":{"sends":0}}"#);
    let response = set_subaccount_limit(&config, "fixture", "fixture-key", LimitInput::new(0))
        .await
        .unwrap();
    assert!(
        matches!(response.entity,Some(SetSubaccountLimitSuccess::Status200(result)) if result.limit.as_ref().unwrap().sends == 0)
    );
    let request = server.join().unwrap();
    assert!(request.starts_with("PUT /sub-account/fixture/limit HTTP/1.1"));
    let body: serde_json::Value =
        serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(body, serde_json::json!({"sends":0}));
    assert!(serde_json::from_str::<LimitInput>(r#"{"sends":null}"#).is_err());
    assert!(serde_json::from_str::<LimitInput>("{}").is_err());
}
#[tokio::test]
async fn clear_limit_is_delete_with_no_json_body() {
    let (config, server) = fixture(204, "");
    let response = delete_subaccount_limit(&config, "fixture", "fixture-key")
        .await
        .unwrap();
    assert!(matches!(
        response.entity,
        Some(DeleteSubaccountLimitSuccess::Status204())
    ));
    let request = server.join().unwrap();
    assert!(request.starts_with("DELETE /sub-account/fixture/limit HTTP/1.1"));
    assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
}
#[tokio::test]
async fn created_key_retained_but_wrapper_debug_redacted() {
    let (config, server) = fixture(201, r#"{"id":41,"key":"fixture-created-secret"}"#);
    let response = create_subaccount_api_key(&config, "fixture", "fixture-key")
        .await
        .unwrap();
    assert!(!format!("{response:?}").contains("fixture-created-secret"));
    assert!(
        matches!(response.entity,Some(CreateSubaccountApiKeySuccess::Status201(key)) if key.id==Some(41) && key.key.as_deref()==Some("fixture-created-secret"))
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("POST /sub-account/fixture/api-key HTTP/1.1"));
}
#[tokio::test]
async fn key_list_pagination_and_delete() {
    let (config, server) = fixture(200, r#"[{"id":41}]"#);
    let response = list_subaccount_api_keys(&config, "fixture", "fixture-key", Some(10), Some(20))
        .await
        .unwrap();
    assert!(
        matches!(response.entity,Some(ListSubaccountApiKeysSuccess::Status200(keys)) if keys.len()==1 && keys[0].id==Some(41) && keys[0].key.is_none())
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /sub-account/fixture/api-key?limit=10&offset=20 HTTP/1.1"));
    let (config, server) = fixture(204, "");
    let response = delete_subaccount_api_key(&config, "fixture", 41, "fixture-key")
        .await
        .unwrap();
    assert!(matches!(
        response.entity,
        Some(DeleteSubaccountApiKeySuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("DELETE /sub-account/fixture/api-key/41 HTTP/1.1"));
}
#[tokio::test]
async fn missing_subaccount_retains_404_without_requiring_json() {
    let (config, server) = fixture(404, "fixture missing");
    match get_subaccount_limit(&config, "fixture", "fixture-key")
        .await
        .unwrap_err()
    {
        Error::ResponseError(response) => {
            assert_eq!(response.status.as_u16(), 404);
            assert!(matches!(
                response.entity,
                Some(GetSubaccountLimitError::Status404())
            ));
            assert_eq!(response.content, "fixture missing");
        }
        _ => panic!("Expected HTTP error"),
    }
    server.join().unwrap();
}

#[tokio::test]
async fn create_subaccount_preserves_explicit_fields() {
    let (config, server) = fixture(
        201,
        r#"{"handle":"fixture","enabled":true,"company_name":"Fixture Company"}"#,
    );
    let mut data = mailchannels_email_api::models::SubAccountData::new("Fixture Company".into());
    data.handle = Some("fixture".into());
    let response = create_subaccount(&config, "fixture-key", Some(data))
        .await
        .unwrap();
    assert!(
        matches!(response.entity,Some(CreateSubaccountSuccess::Status201(account)) if account.handle=="fixture" && account.enabled)
    );
    let request = server.join().unwrap();
    assert!(request.starts_with("POST /sub-account HTTP/1.1"));
    let body: serde_json::Value =
        serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(
        body,
        serde_json::json!({"company_name":"Fixture Company","handle":"fixture"})
    );
}
#[tokio::test]
async fn omitted_optional_body_is_absent_not_json_null() {
    let (config, server) = fixture(201, r#"{"handle":"generated","enabled":true}"#);
    create_subaccount(&config, "fixture-key", None)
        .await
        .unwrap();
    let request = server.join().unwrap();
    assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
    assert!(!request
        .to_lowercase()
        .contains("content-type: application/json"));
}
#[tokio::test]
async fn list_subaccounts_preserves_pagination_and_disabled_state() {
    let (config, server) = fixture(
        200,
        r#"[{"handle":"fixture","enabled":false,"company_name":"Fixture Company"}]"#,
    );
    let response = list_subaccounts(&config, "fixture-key", Some(5), Some(10))
        .await
        .unwrap();
    assert!(
        matches!(response.entity,Some(ListSubaccountsSuccess::Status200(accounts)) if accounts.len()==1 && !accounts[0].enabled && accounts[0].handle=="fixture")
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /sub-account?limit=5&offset=10 HTTP/1.1"));
}
#[tokio::test]
async fn account_lifecycle_uses_distinct_routes_and_empty_204() {
    let (config, server) = fixture(204, "");
    assert!(matches!(
        activate_subaccount(&config, "fixture", "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(ActivateSubaccountSuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("POST /sub-account/fixture/activate HTTP/1.1"));
    let (config, server) = fixture(204, "");
    assert!(matches!(
        suspend_subaccount(&config, "fixture", "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(SuspendSubaccountSuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("POST /sub-account/fixture/suspend HTTP/1.1"));
    let (config, server) = fixture(204, "");
    assert!(matches!(
        delete_subaccount(&config, "fixture", "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(DeleteSubaccountSuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("DELETE /sub-account/fixture HTTP/1.1"));
}
#[tokio::test]
async fn usage_preserves_large_counts_and_billing_dates() {
    let (config, server) = fixture(
        200,
        r#"{"monthly_limit":100,"total_usage":4294967296,"period_start_date":"2026-10-01","period_end_date":"2026-10-31"}"#,
    );
    match get_subaccount_usage(&config, "fixture", "fixture-key")
        .await
        .unwrap()
        .entity
        .unwrap()
    {
        GetSubaccountUsageSuccess::Status200(usage) => {
            assert_eq!(usage.total_usage, 4294967296);
            assert_eq!(usage.monthly_limit, 100);
            assert_eq!(usage.period_start_date.unwrap().to_string(), "2026-10-01");
            assert_eq!(usage.period_end_date.unwrap().to_string(), "2026-10-31");
        }
        _ => panic!("Wrong usage model"),
    }
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /sub-account/fixture/usage HTTP/1.1"));
}
#[tokio::test]
async fn smtp_password_create_list_and_delete() {
    let (config, server) = fixture(
        201,
        r#"{"id":12,"enabled":true,"smtp_password":"fixture-smtp-secret"}"#,
    );
    let response = create_subaccount_smtp_password(&config, "fixture", "fixture-key")
        .await
        .unwrap();
    assert!(!format!("{response:?}").contains("fixture-smtp-secret"));
    assert!(
        matches!(response.entity,Some(CreateSubaccountSmtpPasswordSuccess::Status201(password)) if password.id==Some(12) && password.smtp_password.as_deref()==Some("fixture-smtp-secret") && password.enabled==Some(true))
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("POST /sub-account/fixture/smtp-password HTTP/1.1"));
    let (config, server) = fixture(200, r#"[{"id":12,"enabled":false}]"#);
    assert!(
        matches!(list_subaccount_smtp_passwords(&config,"fixture","fixture-key").await.unwrap().entity,Some(ListSubaccountSmtpPasswordsSuccess::Status200(passwords)) if passwords.len()==1 && passwords[0].enabled==Some(false) && passwords[0].smtp_password.is_none())
    );
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /sub-account/fixture/smtp-password HTTP/1.1"));
    let (config, server) = fixture(204, "");
    assert!(matches!(
        delete_subaccount_smtp_password(&config, "fixture", 12, "fixture-key")
            .await
            .unwrap()
            .entity,
        Some(DeleteSubaccountSmtpPasswordSuccess::Status204())
    ));
    assert!(server
        .join()
        .unwrap()
        .starts_with("DELETE /sub-account/fixture/smtp-password/12 HTTP/1.1"));
}
