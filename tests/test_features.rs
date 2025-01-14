use loco_minijinja_engine::{
    MinijinjaViewEngineConfigurableInitializer, MinijinjaViewEngineInitializer,
};
use loco_rs::app::Initializer;

fn basic_features() {
    let std_initializer = MinijinjaViewEngineInitializer;
    assert_eq!(std_initializer.name(), "minijinja");

    let cfg_initializer =
        MinijinjaViewEngineConfigurableInitializer::new("tests".to_string(), None);
    assert_eq!(cfg_initializer.name(), "minijinja");
}

#[cfg(not(feature = "autoreloader"))]
#[test]
fn test_without_feature_autoreloader() {
    basic_features();
}

#[cfg(feature = "autoreloader")]
#[test]
fn test_with_feature_autoreloader() {
    basic_features();
}
