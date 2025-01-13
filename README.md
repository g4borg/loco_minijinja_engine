# Minijinja Engine for Loco.rs

This crate allows you to integrate [Minijinja](https://github.com/mitsuhiko/minijinja) as Template renderer into [Loco.rs](https://github.com/loco-rs/loco)

## Usage

### Use autoreloader

The `autoreloader` feature automatically uses the `minijinja-autoreloader` instead of a single environment.

Just set in your Cargo.toml:

```toml
loco-minijinja-engine = { features = ["autoreloader"] }
```

### Default Settings

If you want the standard initializer, so access `"assets/templates"` as your template directory, and a standard minijinja renderer as it comes out of the box, just use in your `app.rs`:

```rust
...

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(loco_minijinja_engine::MinijinjaViewEngineInitializer)])
    }

...
```

### Custom Environment or Template Directory

If you want a different directory for your templates, e.g. stay with `"assets/views"` like the Tera setup in loco, use the custom initializer in your `app.rs`:

```rust
...

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        let environment = Environment::new();
        Ok(vec![Box::new(
            loco_minijinja_engine::MinijinjaViewEngineConfigurableInitializer::new(
                "assets/views".to_string(),
                Some(environment),
            ),
        )])
    }
```

(Note, that because of the trait layout, I cannot prevent two clones and 'static for the custom Environment, so if you need anything more dynamic, feel free to copypaste the initializer, and initialize the Environment in the closure)
