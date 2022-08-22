# Egui notify
Simple notifications library for EGUI

# Usage
```rust
use egui_notify::Toasts;

let mut t = Toasts::default();
t.info("Hello world!", |t| t.with_duration(5.));
// ...
t.show(ctx);
```

# Installation
```toml
[dependencies]
egui-notify = "0.1"
```