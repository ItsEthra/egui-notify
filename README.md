# Egui notify
Simple notifications library for EGUI

# Usage
```rust
use egui_notify::Toasts;
use std::time::Duration;

let mut t = Toasts::default();
t.info("Hello world!").set_duration(Duration::from_secs(5));
// ...
t.show(ctx);
```

# Installation
```toml
[dependencies]
egui-notify = "0.2"
```