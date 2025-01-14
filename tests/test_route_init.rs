#[cfg(test)]
#[cfg(feature = "testing")]
mod tests {
    use axum::{routing::get, Router};
    use loco_minijinja_engine::MinijinjaViewEngineConfigurableInitializer;
    use loco_rs::app::Initializer;
    use loco_rs::tests_cfg;

    #[tokio::test]
    async fn test_after_routes_success() {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let ctx = tests_cfg::app::get_app_context().await;

        let std_initializer =
            MinijinjaViewEngineConfigurableInitializer::new("tests".to_string(), None);

        // Call the after_routes function
        let result = std_initializer.after_routes(router, &ctx).await;

        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
    }
}
