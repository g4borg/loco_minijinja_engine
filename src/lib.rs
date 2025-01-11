use async_trait::async_trait;
use axum::{response::Html, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    controller::views::{ViewEngine, ViewRenderer},
    errors::Error,
    Result,
};
use minijinja::{filters::default, path_loader, Environment};
#[cfg(feature = "autoreloader")]
use minijinja_autoreload::AutoReloader;
use serde::Serialize;
use std::{marker::PhantomData, path::Path, sync::Arc};

const TEMPLATES_DIR: &str = "assets/templates";

#[derive(Clone)]
pub struct MinijinjaView<'a> {
    #[cfg(debug_assertions)]
    pub template_dir: String,
    #[cfg(not(feature = "autoreloader"))]
    pub environment: Arc<std::sync::Mutex<Environment<'a>>>,
    #[cfg(feature = "autoreloader")]
    pub reloader: Arc<std::sync::Mutex<minijinja_autoreload::AutoReloader>>,

    phantom: PhantomData<&'a ()>,
}

impl MinijinjaView<'_> {
    pub fn build() -> Result<Self> {
        Self::from_custom_dir(&TEMPLATES_DIR)
    }

    pub fn from_custom_dir<P: AsRef<Path>>(path: &P) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(Error::string(&format!(
                "missing templates directory: `{}`",
                path.as_ref().display()
            )));
        }

        let template_path = path.as_ref().to_string_lossy().to_string();

        #[cfg(feature = "autoreloader")]
        let reloader = AutoReloader::new(move |notifier| {
            let mut environment = Environment::new();
            let template_path = template_path.clone();

            environment.set_loader(path_loader(&template_path));

            notifier.watch_path(template_path, true);
            Ok(environment)
        });

        #[cfg(not(feature = "autoreloader"))]
        let environment = {
            let mut environment = Environment::new();
            environment.set_loader(path_loader(&template_path));
            environment
        };

        Ok(Self {
            #[cfg(debug_assertions)]
            template_dir: path.as_ref().to_string_lossy().to_string(),
            #[cfg(feature = "autoreloader")]
            reloader: std::sync::Arc::new(std::sync::Mutex::new(reloader)),
            #[cfg(not(feature = "autoreloader"))]
            environment: std::sync::Arc::new(std::sync::Mutex::new(environment)),
            phantom: Default::default(),
        })
    }

    pub fn render_template<D: serde::Serialize>(
        &self,
        key: &str,
        data: D,
    ) -> Result<String, Error> {
        #[cfg(feature = "autoreloader")]
        let reloader = self.reloader.lock().expect("reloader lock failed");
        let env = {
            #[cfg(feature = "autoreloader")]
            {
                reloader
                    .acquire_env()
                    .or_else(|e| Err(Error::Message(e.to_string())))?
            }
            #[cfg(not(feature = "autoreloader"))]
            self.environment.lock().expect("environment lock failed")
        };

        let template = env
            .get_template(key)
            .or_else(|e| Err(Error::Message(e.to_string())))?;
        let rendered = template
            .render(&data)
            .or_else(|e| Err(Error::Message(e.to_string())))?;
        Ok(rendered)
    }

    pub fn render_html<D: serde::Serialize>(
        &self,
        key: &str,
        data: D,
    ) -> Result<Html<String>, Error> {
        let result = self.render_template(key, data)?;
        Ok(Html(result))
    }
}

impl ViewRenderer for MinijinjaView<'_> {
    fn render<S: Serialize>(&self, _key: &str, _data: S) -> Result<String> {
        self.render_template(_key, _data)
    }
}

pub struct MinijinjaViewEngineInitializer;

#[async_trait]
impl Initializer for MinijinjaViewEngineInitializer {
    fn name(&self) -> String {
        "minijinja".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let jinja = MinijinjaView::build()?;
        Ok(router.layer(Extension(ViewEngine::from(jinja))))
    }
}
