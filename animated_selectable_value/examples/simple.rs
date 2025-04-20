fn main() -> eframe::Result {
    use eframe::egui;
    use egui_animated_selectable_value::prelude::*;

    #[derive(PartialEq, PartialOrd, Clone, Eq, Hash)]
    enum Page {
        Page1,
        Page2,
        Page3,
        Page4,
    }

    let mut page = Page::Page1;
    let mut page_2 = Page::Page1;

    eframe::run_simple_native(
        "Egui page transition example",
        Default::default(),
        move |ctx, _frame| {
            ctx.style_mut(|style| style.animation_time = 0.2);
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(100.0);

                ui.horizontal(|ui| {
                    animated_selectable_value(ui, 0, &mut page, Page::Page1, "Page 1");
                    animated_selectable_value(ui, 0, &mut page, Page::Page2, "Page 2 aaaaaaaaaa");
                    animated_selectable_value(ui, 0, &mut page, Page::Page3, "Page 3");
                    animated_selectable_value(ui, 0, &mut page, Page::Page4, "Page 4");
                });

                ui.add_space(32.0);

                ui.horizontal(|ui| {
                    animated_selectable_value(ui, 1, &mut page_2, Page::Page1, "Page 1");
                    animated_selectable_value(ui, 1, &mut page_2, Page::Page2, "Page 2 aaaaaaaaaa");
                });
                ui.horizontal(|ui| {
                    animated_selectable_value(ui, 1, &mut page_2, Page::Page3, "Page 3");
                    animated_selectable_value(ui, 1, &mut page_2, Page::Page4, "Page 4");
                });
            });
        },
    )
}
