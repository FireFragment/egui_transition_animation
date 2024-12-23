This crate allows you to have animated transitions between multiple "pages" in [egui](https://github.com/emilk/egui). \
You will mostly use the [`animated_pager`] function.

See [the README](https://github.com/FireFragment/egui_transition_animation/blob/development/README.md) for a video of the animations.

### Quickstart

```rust
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
            );
            # ctx.send_viewport_cmd(egui::ViewportCommand::Close); // To pass doctests
        });
    },
).unwrap();
```
