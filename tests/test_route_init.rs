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
    use minijinja::Environment;
    use serde::Serialize;
    use serde_json::Value;

    #[tokio::test]
    async fn test_after_routes_configured() {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let ctx = tests_cfg::app::get_app_context().await;
        let env: Environment = Environment::new();

        let initializer =
            MinijinjaViewEngineConfigurableInitializer::new("tests".to_string(), Some(env));

        // Call the after_routes function
        let result = initializer.after_routes(router, &ctx).await;

        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
    }

    #[tokio::test]
    async fn test_after_routes_configured_fails() {
        let router = Router::new().route("/", get(|| async { "Hello, World!" }));
        let ctx = tests_cfg::app::get_app_context().await;
        let initializer =
            MinijinjaViewEngineConfigurableInitializer::new("does_not_exist".to_string(), None);
        let result = initializer.after_routes(router, &ctx).await;
        assert!(result.is_err(), "result was unexpectedly OK: {:?}", result);
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
        let result = jinja.render("test_1.html", Value::default());
        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
        assert_eq!(
            result.unwrap().replace("\r", "").replace("\n", ""),
            "Hello World!"
        );
    }

    #[derive(Serialize)]
    struct ReturnContext {
        context: Vec<u8>,
    }

    #[test]
    fn test_rendering_html() {
        let jinja: MinijinjaView = MinijinjaView::build().unwrap();
        let result = jinja.render_html(
            "test_2.html",
            ReturnContext {
                context: vec![1, 2, 3],
            },
        );
        assert!(result.is_ok(), "result was NOT OK: {:?}", result);
        let content: String = result.expect("Should never fail").0;
        assert_eq!(
            content.replace("\r", "").replace("\n", ""),
            "<li>1<li><li>2<li><li>3<li>"
        );
    }

    #[test]
    fn test_rendering_garbage() {
        let jinja: MinijinjaView = MinijinjaView::build().unwrap();
        let result = jinja.render("test_3.html", Value::default());
        assert!(result.is_err(), "result was unexpectedly OK: {:?}", result);
    }
}
