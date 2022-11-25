use std::{
    thread::{self, Thread},
    time::Duration,
};

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label},
    Env, EventCtx, FontDescriptor, FontFamily, KeyOrValue, Widget, WidgetExt,
};
use enigo::{Enigo, KeyboardControllable, MouseControllable};

use crate::{
    atomic::group,
    comms::{Channel, COMMS},
    options::ScanOrder,
    vars::{SIZE_L, SIZE_S, SIZE_XL},
    AppState,
};

fn retract(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn expand(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

fn left(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn right(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

fn up(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn down(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

fn scan(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {
    let mut comms = COMMS.lock().unwrap();
    comms.send(b"ping\0").unwrap();

    let response = comms.read_str().unwrap();
    _data.logs.push_str(response.as_str());

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

const SCAN_DELAY: u64 = 350;
const RIGHT_KEY: enigo::Key = enigo::Key::F9;
const LEFT_KEY: enigo::Key = enigo::Key::F8;
const INAPP_KEY: enigo::Key = enigo::Key::F10;
fn send_key(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(600, 200);
    enigo.mouse_click(enigo::MouseButton::Left);

    match data.scan_order {
        ScanOrder::InApp => enigo.key_click(INAPP_KEY),
        ScanOrder::Right => enigo.key_click(RIGHT_KEY),
        ScanOrder::Left => enigo.key_click(LEFT_KEY),
        ScanOrder::RightThenLeft => {
            thread::spawn(move || {
                enigo.key_click(RIGHT_KEY);
                thread::sleep(Duration::from_millis(SCAN_DELAY));
                enigo.key_click(LEFT_KEY);
            })
            .join()
            .unwrap();
        }
        ScanOrder::LeftThenRight => {
            thread::spawn(move || {
                enigo.key_click(LEFT_KEY);
                thread::sleep(Duration::from_millis(SCAN_DELAY));
                enigo.key_click(RIGHT_KEY);
            })
            .join()
            .unwrap();
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
                                    .on_click(retract)
                                    .expand(),
                                1.,
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Button::from_label(Label::new("←→").with_font(BUTTON_FONT))
                                    .on_click(expand)
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
                                    .on_click(left)
                                    .expand(),
                                FlexParams::new(1., CrossAxisAlignment::End),
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Flex::column()
                                    .with_flex_child(
                                        Button::from_label(Label::new("↑").with_font(BUTTON_FONT))
                                            .on_click(up)
                                            .expand(),
                                        1.,
                                    )
                                    .with_spacer(SIZE_S)
                                    .with_flex_child(
                                        Button::from_label(Label::new("↓").with_font(BUTTON_FONT))
                                            .on_click(down)
                                            .expand(),
                                        1.,
                                    ),
                                1.,
                            )
                            .with_spacer(SIZE_S)
                            .with_flex_child(
                                Button::from_label(Label::new("→").with_font(BUTTON_FONT))
                                    .on_click(right)
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
                    .fix_height(BUTTON_SIZE),
            ),
    );

    template
}
