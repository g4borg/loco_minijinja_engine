#[cfg(test)]
#[cfg(feature = "testing")]
mod tests {
    use axum::{routing::get, Router};
    use loco_minijinja_engine::{
        MinijinjaView, MinijinjaViewEngineConfigurableInitializer, MinijinjaViewEngineInitializer,
    };
    use loco_rs::app::Initializer;
    use loco_rs::controller::views::ViewRenderer;
    use loco_rs::tests_cfg;
    use serde_json::Value;

    #[tokio::test]
    async fn test_after_routes_configured() {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let ctx = tests_cfg::app::get_app_context().await;

        let initializer =
            MinijinjaViewEngineConfigurableInitializer::new("tests".to_string(), None);

        // Call the after_routes function
        let result = initializer.after_routes(router, &ctx).await;

        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
    }

    #[tokio::test]
    async fn test_after_routes_std() {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let ctx = tests_cfg::app::get_app_context().await;

        let initializer = MinijinjaViewEngineInitializer;
        let result = initializer.after_routes(router, &ctx).await;

        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
    }

    #[test]
    fn test_rendering() {
        let jinja: MinijinjaView = MinijinjaView::build().unwrap();
        let result = jinja.render("test.html", Value::default());
        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
        assert_eq!(result.unwrap(), "\r\nHello World!\r\n");
    }
}
