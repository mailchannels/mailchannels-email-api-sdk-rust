use mailchannels_email_api::apis::{metrics_api::*, usage_api::*};
mod common;
use common::fixture;
fn assert_filters(request: String, path: &str) {
    let target = request.split_whitespace().nth(1).unwrap();
    let url = url::Url::parse(&format!("http://fixture{target}")).unwrap();
    assert_eq!(url.path(), path);
    let params: Vec<_> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    assert_eq!(
        params,
        vec![
            ("start_time".into(), "2026-10-01T00:00:00Z".into()),
            ("end_time".into(), "2026-10-02T00:00:00Z".into()),
            ("campaign_id".into(), "fixture+campaign".into()),
            ("interval".into(), "day".into())
        ]
    );
}
macro_rules! metric {
    ($name:ident,$function:ident,$response:ident,$path:literal,$body:literal,$field:literal,$expected:expr) => {
        #[tokio::test]
        async fn $name() {
            let (config, server) = fixture(200, $body);
            let response = $function(
                &config,
                "fixture-key",
                Some("2026-10-01T00:00:00Z"),
                Some("2026-10-02T00:00:00Z"),
                Some("fixture+campaign"),
                Some("day"),
            )
            .await
            .unwrap();
            match response.entity.unwrap() {
                $response::Status200(metrics) => {
                    let actual = serde_json::to_value(metrics).unwrap();
                    let expected: serde_json::Value = serde_json::from_str($body).unwrap();
                    assert_eq!(actual[$field], $expected);
                    assert_eq!(actual["buckets"], expected["buckets"]);
                }
                _ => panic!("Wrong metrics model"),
            }
            assert_filters(server.join().unwrap(), $path);
        }
    };
}
metric!(
    engagement_preserves_unique_counts,
    get_engagement_metrics,
    GetEngagementMetricsSuccess,
    "/metrics/engagement",
    r#"{"open":3,"open_tracking_delivered":4,"click":2,"click_tracking_delivered":4,"unique_open":2,"unique_click":1,"buckets":{"open":[{"period_start":"2026-10-01T00:00:00Z","count":3}],"open_tracking_delivered":[],"click":[],"click_tracking_delivered":[]}}"#,
    "unique_click",
    1
);
metric!(
    performance_preserves_bounce_counts,
    get_performance_metrics,
    GetPerformanceMetricsSuccess,
    "/metrics/performance",
    r#"{"delivered":10,"bounced":2,"complained":1,"processed":12,"buckets":{"delivered":[],"bounced":[{"period_start":"2026-10-01T00:00:00Z","count":2}],"complained":[],"processed":[]}}"#,
    "bounced",
    2
);
metric!(
    recipient_metrics_preserve_unsubscribes,
    get_recipient_behaviour_metrics,
    GetRecipientBehaviourMetricsSuccess,
    "/metrics/recipient-behaviour",
    r#"{"unsubscribed":2,"unsubscribe_delivered":10,"buckets":{"unsubscribed":[{"period_start":"2026-10-01T00:00:00Z","count":2}],"unsubscribe_delivered":[]}}"#,
    "unsubscribed",
    2
);
metric!(
    volume_preserves_dropped_count,
    get_volume_metrics,
    GetVolumeMetricsSuccess,
    "/metrics/volume",
    r#"{"processed":12,"delivered":10,"dropped":2,"buckets":{"processed":[],"delivered":[],"dropped":[{"period_start":"2026-10-01T00:00:00Z","count":2}]}}"#,
    "dropped",
    2
);
#[tokio::test]
async fn sender_metrics_preserve_sort_pagination_and_totals() {
    let (config, server) = fixture(
        200,
        r#"{"limit":5,"offset":10,"total":20,"senders":[{"name":"fixture-campaign","processed":10,"delivered":8,"bounced":1,"dropped":1}]}"#,
    );
    match get_sender_metrics(
        &config,
        "campaigns",
        "fixture-key",
        None,
        None,
        Some(5),
        Some(10),
        Some("desc"),
    )
    .await
    .unwrap()
    .entity
    .unwrap()
    {
        GetSenderMetricsSuccess::Status200(metrics) => {
            assert_eq!(metrics.total, 20);
            assert_eq!(metrics.limit, 5);
            assert_eq!(metrics.offset, 10);
            assert_eq!(metrics.senders[0].name, "fixture-campaign");
            assert_eq!(metrics.senders[0].delivered, 8);
        }
        _ => panic!("Wrong sender metrics"),
    }
    assert!(server
        .join()
        .unwrap()
        .starts_with("GET /metrics/senders/campaigns?limit=5&offset=10&sort_order=desc HTTP/1.1"));
}
#[tokio::test]
async fn parent_usage_preserves_zero_limit_and_large_total() {
    let (config, server) = fixture(
        200,
        r#"{"monthly_limit":0,"total_usage":4294967296,"period_start_date":"2026-10-01","period_end_date":"2026-10-31"}"#,
    );
    assert!(
        matches!(get_usage(&config,"fixture-key").await.unwrap().entity,Some(GetUsageSuccess::Status200(usage)) if usage.monthly_limit==0 && usage.total_usage==4294967296)
    );
    assert!(server.join().unwrap().starts_with("GET /usage HTTP/1.1"));
}
