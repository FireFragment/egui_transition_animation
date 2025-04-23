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

    let mut page_1 = Page::Page1;
    let mut page_2 = Page::Page1;
    let mut page_3 = Page::Page1;

    let mut show = false;

    eframe::run_simple_native(
        "Egui page transition example",
        Default::default(),
        move |ctx, _frame| {
            ctx.style_mut(|style| style.animation_time = 0.2);
            egui::CentralPanel::default().show(ctx, |ui| {
                if !show {
                    if ui.button("Show").clicked() {
                        show = true;
                    }

                    return;
                }

                ui.horizontal(|ui| {
                    let mut tabs =
                        begin_animated_selectable_value(ui, 1, Default::default(), &mut page_1);
                    tabs.value(ui, Page::Page1, "Page 1");
                    tabs.value(ui, Page::Page2, "Page 2 with a long name");
                    tabs.value(ui, Page::Page3, "Page 3");
                    tabs.value(ui, Page::Page4, "Page 4");
                });

                ui.add_space(32.0);

                let mut tabs =
                    begin_animated_selectable_value(ui, 2, Default::default(), &mut page_2);

                ui.horizontal(|ui| {
                    tabs.value(ui, Page::Page1, "Page 1");
                    tabs.value(ui, Page::Page2, "Page 2 with a long name");
                });
                ui.horizontal(|ui| {
                    tabs.value(ui, Page::Page3, "Page 3");
                    tabs.value(ui, Page::Page4, "Page 4");
                });

                ui.add_space(32.0);

                ui.vertical_centered_justified(|ui| {
                    let mut tabs =
                        begin_animated_selectable_value(ui, 3, Default::default(), &mut page_3);
                    tabs.value(ui, Page::Page1, "Page 1");
                    tabs.value(ui, Page::Page2, "Page 2 with a long name");
                    tabs.value(ui, Page::Page3, "Page 3");
                    tabs.value(ui, Page::Page4, "Page 4");
                });
            });
        },
    )
}
