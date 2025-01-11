use loco_rs::testing::prelude::*;
use serial_test::serial;
use usage_example::app::App;

#[tokio::test]
#[serial]
async fn can_get_home() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api").await;

        assert_eq!(res.status_code(), 200);
        res.assert_json(&serde_json::json!({"app_name":"loco"}));
    })
    .await;
}