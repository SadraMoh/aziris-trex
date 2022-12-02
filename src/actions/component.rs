use std::{thread, time::Duration};

use druid::{
    widget::{Button, Either, Flex, Label, ProgressBar},
    Env, EventCtx, Widget, WidgetExt,
};

use crate::{
    atomic::group,
    comms::{commands, COMMS},
    vars::SIZE_S,
    AppState,
};

fn auto_adjust(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn calibrate(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();

    if data.is_calibrating {
        // cancel calibration
        comms.cmd(commands::CALIBRATION_STOP).unwrap();
    } else {
        // start calibration
        comms.cmd(commands::CALIBRATION_START).unwrap();
    }

    data.is_calibrating ^= true; // flip is calibrating
}

pub fn build_actions() -> impl Widget<AppState> {
    let template = group(
        "Actions",
        Flex::row()
            // .with_flex_child(Button::new("Automatic adjustment"), 1.)
            // .with_flex_child(Button::new("Calibrate"), 1.),
            .must_fill_main_axis(true)
            .with_child(Button::new("Automatic adjustment").on_click(auto_adjust))
            .with_spacer(SIZE_S)
            .with_child(
                Button::from_label(Label::new(|is_calibrating: &bool, _: &Env| {
                    if *is_calibrating {
                        "Stop Calibration"
                    } else {
                        "Calibrate"
                    }
                }))
                .lens(AppState::is_calibrating)
                .on_click(calibrate),
            )
            .with_spacer(SIZE_S)
            .with_child(Either::new(
                |data: &AppState, _: &Env| data.is_calibrating,
                ProgressBar::new().lens(AppState::counter),
                Label::new(""),
            ))
            .expand_width(),
    )
    .expand_width();

    template
}
