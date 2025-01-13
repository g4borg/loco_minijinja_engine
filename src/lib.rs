use async_trait::async_trait;
use axum::{response::Html, Extension, Router as AxumRouter};
use loco_rs::{
    app::{AppContext, Initializer},
    controller::views::{ViewEngine, ViewRenderer},
    errors::Error,
    Result,
};
use minijinja::{path_loader, Environment};
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
        Self::from_custom(&TEMPLATES_DIR, None)
    }

    pub fn from_custom<P: AsRef<Path>>(
        path: &P,
        custom_environment: Option<Environment<'static>>,
    ) -> Result<Self> {
        if !path.as_ref().exists() {
            return Err(Error::string(&format!(
                "missing templates directory: `{}`",
                path.as_ref().display()
            )));
        }

        let template_path = path.as_ref().to_string_lossy().to_string();

        #[cfg(feature = "autoreloader")]
        let reloader = AutoReloader::new(move |notifier| {
            let cust_env = custom_environment.clone();
            let mut env = cust_env.unwrap_or_else(Environment::new);
            let template_path = template_path.clone();
            env.set_loader(path_loader(&template_path));
            notifier.watch_path(template_path, true);
            Ok(env)
        });

        #[cfg(not(feature = "autoreloader"))]
        let environment = {
            let mut env = custom_environment.unwrap_or_else(Environment::new);
            env.set_loader(path_loader(&template_path));
            env
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

pub struct MinijinjaViewEngineConfigurableInitializer {
    template_directory: String,
    custom_environment: Option<Environment<'static>>,
}

#[async_trait]
impl Initializer for MinijinjaViewEngineConfigurableInitializer {
    fn name(&self) -> String {
        "minijinja".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let custom_environment = self.custom_environment.clone(); // as this is a &self function, we have to clone here.
        let jinja =
            MinijinjaView::from_custom::<String>(&self.template_directory, custom_environment)?;
        Ok(router.layer(Extension(ViewEngine::from(jinja))))
    }
}

impl MinijinjaViewEngineConfigurableInitializer {
    pub fn new(
        template_directory: String,
        custom_environment: Option<Environment<'static>>,
    ) -> Self {
        MinijinjaViewEngineConfigurableInitializer {
            template_directory: template_directory,
            custom_environment: custom_environment,
        }
    }
}
