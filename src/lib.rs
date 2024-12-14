use eframe::{
    egui::{self, Ui},
    emath::easing,
};

pub fn page_transition(ui: &mut Ui, t: f32, mut add_contents: impl FnMut(&mut Ui, bool)) {
    let dist = 16.0;
    if t <= 0.5 {
        let space = -dist * t * 2.;
        ui.add_space(space);
        add_contents(ui, false);
    } else {
        let tf = 2. * t - 1.;
        let space = dist + -dist * tf;
        ui.add_space(space);
        add_contents(ui, true);
    }
}
