use druid::{
    widget::{Button, Either, Flex, Label, Spinner},
    Env, EventCtx, Widget, WidgetExt,
};

use crate::{
    atomic::group,
    comms::{commands, COMMS},
    vars::SIZE_S,
    AppState,
};

fn auto_adjust(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::ADJUST_START).unwrap();
}

fn calibrate(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();

    if data.is_calibrating {
        // cancel calibration
        comms.cmd(commands::CALIBRATION_STOP).unwrap();
    } else {
        // start calibration
        comms.cmd(commands::CALIBRATION_START).unwrap();
    }
}

pub fn build_actions() -> impl Widget<AppState> {
    let template = group(
        "Actions",
        Flex::column()
            .must_fill_main_axis(true)
            .with_child(
                Flex::row()
                    .with_child(Button::new("Automatic adjustment").on_click(auto_adjust))
                    .with_spacer(SIZE_S)
                    .with_child(Either::new(
                        |data: &AppState, _: &Env| data.is_auto_adjusting,
                        Spinner::new(),
                        Label::new(""),
                    )).align_left(),
            )
            .with_spacer(SIZE_S)
            .with_child(
                Flex::row()
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
                        Spinner::new(),
                        Label::new(""),
                    ))
                    .align_left(),
            )
            .expand_width(),
    )
    .expand_width();

    template
}
