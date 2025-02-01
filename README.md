# Egui transition animation

This crate allows you to have animated transitions between multiple "pages" in [egui](https://github.com/emilk/egui).

[crates.io](https://crates.io/crates/egui_transition_animation) | [docs.rs](https://docs.rs/egui_transition_animation/0.1.1/egui_transition_animation/) | [lib.rs](https://lib.rs/crates/egui_transition_animation) | [GitHub](https://github.com/FireFragment/egui_transition_animation)

[egui_transition.webm](https://github.com/user-attachments/assets/26fec04d-9b03-4cd5-8504-9ca0b5b85fc9)

### Quickstart

```rust
fn main() -> eframe::Result {
    use eframe::egui;
    use egui_transition_animation::prelude::*;

    #[derive(PartialEq, PartialOrd, Clone, Eq)]
    enum Page {
        Page1,
        Page2,
        Page3,
    }

    let mut page = Page::Page1;

    eframe::run_simple_native(
        "Egui transition animation example",
        Default::default(),
        move |ctx, _frame| {
            ctx.style_mut(|style| style.animation_time = 0.2);
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut page, Page::Page1, "Page 1");
                    ui.selectable_value(&mut page, Page::Page2, "Page 2");
                    ui.selectable_value(&mut page, Page::Page3, "Page 3");
                });

                animated_pager(
                    ui,
                    page.clone(),
                    &TransitionStyle::horizontal(ui),
                    egui::Id::new("pager"),
                    |ui, page| match page {
                        Page::Page1 => ui.label("Hello from page 1"),
                        Page::Page2 => ui.heading("Hello from page 2"),
                        Page::Page3 => ui.monospace("Hello from page 3"),
                    },
                )
            });
        },
    )
}
```

### Contributing

Contributions are welcome. Please don't forget to make PRs against the `development` branch not `release`.