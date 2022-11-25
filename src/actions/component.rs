use druid::{
    widget::{Button, Flex},
    Env, EventCtx, Widget,
};

use crate::{
    atomic::group,
    vars::{ SIZE_S},
    AppState,
};

fn auto_adjust(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn calibrate(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

pub fn build_actions() -> impl Widget<AppState> {
    let template = group(
        "Actions",
        Flex::row()
            // .with_flex_child(Button::new("Automatic adjustment"), 1.)
            // .with_flex_child(Button::new("Calibrate"), 1.),
            .must_fill_main_axis(true)
            .with_child(Button::new("Automatic adjustment").on_click(auto_adjust))
            .with_spacer(SIZE_S)
            .with_child(Button::new("Calibrate").on_click(calibrate)),
    );

    template
}
