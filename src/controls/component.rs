use std::{thread, time::Duration};

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label},
    Env, Event, EventCtx, FontDescriptor, FontFamily, KeyEvent, KeyOrValue, Widget, WidgetExt, LifeCycleCtx,
};
use enigo::{Enigo, KeyboardControllable, MouseControllable};

use crate::{
    atomic::group,
    comms::{commands, COMMS},
    custom_controllers::{ButtonState, KeyLogger},
    options::{ScanMode, ScanOrder},
    vars::{SIZE_L, SIZE_S, SIZE_XL},
    AppState,
};

// #region METHODS

fn retract(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_CLOSE).unwrap();
}

fn retract_stop(
    _ctx: &mut druid::EventCtx,
    _event: &druid::Event,
    _data: &mut AppState,
    _env: &Env,
) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_CLOSE_STOP).unwrap();
}

fn expand(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_OPEN).unwrap();
}

fn expand_stop(
    _ctx: &mut druid::EventCtx,
    _event: &druid::Event,
    _data: &mut AppState,
    _env: &Env,
) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_OPEN_STOP).unwrap();
}

fn left(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_LEFT).unwrap();
}

fn left_stop(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_LEFT_STOP).unwrap();
}

fn right(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_RIGHT).unwrap();
}

fn right_stop(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_RIGHT_STOP).unwrap();
}

fn up(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_UP).unwrap();
}

fn up_stop(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_UP_STOP).unwrap();
}

fn down(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_DOWN).unwrap();
}

fn down_stop(_ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::CRADLE_DOWN_STOP).unwrap();
}

fn scan(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::PING).unwrap();

    send_key(_data);
}

// #endregion METHODS

const SCAN_DELAY: u64 = 500;
const RIGHT_KEY: enigo::Key = enigo::Key::F9;
const LEFT_KEY: enigo::Key = enigo::Key::F8;
const INAPP_KEY: enigo::Key = enigo::Key::F10;
const INAPP2_KEY: enigo::Key = enigo::Key::F7;
pub fn send_key(data: &mut AppState) {
    let mut enigo = Enigo::new();

    // enigo.mouse_move_to(600, 200);
    // enigo.mouse_click(enigo::MouseButton::Left);

    match data.scan_order {
        ScanOrder::InApp => enigo.key_click(INAPP_KEY),
        ScanOrder::InApp2 => enigo.key_click(INAPP2_KEY),
        ScanOrder::Right => enigo.key_click(RIGHT_KEY),
        ScanOrder::Left => enigo.key_click(LEFT_KEY),
        ScanOrder::RightThenLeft => {
            thread::spawn(move || {
                enigo.key_click(RIGHT_KEY);
                thread::sleep(Duration::from_millis(SCAN_DELAY));
                enigo.key_click(LEFT_KEY);
            });
        }
        ScanOrder::LeftThenRight => {
            thread::spawn(move || {
                enigo.key_click(LEFT_KEY);
                thread::sleep(Duration::from_millis(SCAN_DELAY));
                enigo.key_click(RIGHT_KEY);
            });
        }
    };
}

const BUTTON_SIZE: f64 = 72.;
const BUTTON_FONT: KeyOrValue<FontDescriptor> =
    KeyOrValue::Concrete(FontDescriptor::new(FontFamily::MONOSPACE).with_size(22.));

pub fn build_controls() -> impl Widget<AppState> {
    let template = group(
        "Controls",
        Flex::column()
            .with_child(
                Flex::row()
                    .must_fill_main_axis(true)
                    .with_flex_child(
                        Flex::column()
                            .with_flex_child(
                                Button::from_label(Label::new("→←").with_font(BUTTON_FONT))
                                    .controller(ButtonState::new(retract, retract_stop))
                                    .expand(),
                                1.,
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Button::from_label(Label::new("←→").with_font(BUTTON_FONT))
                                    .controller(ButtonState::new(expand, expand_stop))
                                    .expand(),
                                1.,
                            ),
                        1.,
                    )
                    .with_spacer(SIZE_XL)
                    .with_flex_child(
                        Flex::row()
                            .with_flex_child(
                                Button::from_label(Label::new("←").with_font(BUTTON_FONT))
                                    .controller(ButtonState::new(left, left_stop))
                                    .expand(),
                                FlexParams::new(1., CrossAxisAlignment::End),
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Flex::column()
                                    .with_flex_child(
                                        Button::from_label(Label::new("↑").with_font(BUTTON_FONT))
                                            .controller(ButtonState::new(up, up_stop))
                                            .expand(),
                                        1.,
                                    )
                                    .with_spacer(SIZE_S)
                                    .with_flex_child(
                                        Button::from_label(Label::new("↓").with_font(BUTTON_FONT))
                                            .controller(ButtonState::new(down, down_stop))
                                            .expand(),
                                        1.,
                                    ),
                                1.,
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Button::from_label(Label::new("→").with_font(BUTTON_FONT))
                                    .controller(ButtonState::new(right, right_stop))
                                    .expand(),
                                FlexParams::new(1., CrossAxisAlignment::End),
                            ),
                        3.,
                    )
                    .fix_height(BUTTON_SIZE),
            )
            .with_spacer(SIZE_L)
            .with_child(
                Button::from_label(Label::new("Scan").with_font(BUTTON_FONT))
                    .on_click(scan)
                    .expand_width()
                    .fix_height(BUTTON_SIZE)
                    .disabled_if(|data: &AppState, _: &Env| data.scan_mode != ScanMode::Panel),
            ),
    );

    template
}

fn _ping(_data: &mut AppState) {
    let mut comms = COMMS.lock().unwrap();
    comms.cmd(commands::PING).unwrap();

    thread::spawn(move || {
        let mut enigo = Enigo::new();
        thread::sleep(Duration::from_millis(5000));
        enigo.mouse_move_to(600, 200);
        enigo.mouse_click(enigo::MouseButton::Left);
        enigo.key_click(RIGHT_KEY);
        enigo.key_click(LEFT_KEY);
    })
    .join()
    .unwrap();
}
