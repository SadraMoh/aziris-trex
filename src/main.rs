#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use druid::widget::{CrossAxisAlignment, Flex, FlexParams, Padding};
use druid::{AppLauncher, Env, EventCtx, PlatformError, Widget, WindowDesc};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use trex_ui::actions::build_actions;
use trex_ui::connection_status::build_connection_status;
use trex_ui::controls::build_controls;
use trex_ui::lights::build_lights;
use trex_ui::logs::build_logs;
use trex_ui::options::{build_options, ScanOrder};
use trex_ui::vars::{SIZE_L, SIZE_XXL};
use trex_ui::AppState;



fn main() -> Result<(), PlatformError> {

    println!("INIT");
    
    let main_window = WindowDesc::new(ui_builder())
        .title("T-Rex Control Panel")
        .window_size((900., 900.))
        .with_min_size((640., 540.));

    let data = AppState::default();

    Ok(AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch the application"))
}

fn ui_builder() -> impl Widget<AppState> {
    let template = Padding::new(
        12.,
        Flex::row()
            // left
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
                    .must_fill_main_axis(true)
                    .with_child(build_connection_status())
                    .with_spacer(SIZE_XXL)
                    .with_child(build_options())
                    .with_spacer(SIZE_L)
                    .with_child(build_actions())
                    .with_spacer(SIZE_L)
                    .with_child(build_lights())
                    .with_spacer(SIZE_L)
                    .with_child(build_controls())
                    // test buttons
                    // .with_child(Button::new("Test").on_click(count_up))
                    // .with_child(Button::new("Hello World").on_click(autogui_hello_world))
                    // .with_child(Button::new("Scan").on_click(send_key))
                    , // .with_child(radios)
                1.,
            )
            .with_spacer(SIZE_XXL)
            // right
            .with_flex_child(build_logs(), FlexParams::new(1., CrossAxisAlignment::End)),
    );

    template
}
    
fn autogui_hello_world(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(600, 200);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_sequence_parse("hello world");
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
            }).join().unwrap();
        }
        ScanOrder::LeftThenRight => {
            thread::spawn(move || {
                enigo.key_click(LEFT_KEY);
                thread::sleep(Duration::from_millis(SCAN_DELAY));
                enigo.key_click(RIGHT_KEY);
            }).join().unwrap();
        }
    };
}

fn count_up(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.counter += 1;
    data.scan_order = match data.scan_order {
        ScanOrder::Left => ScanOrder::Right,
        ScanOrder::Right => ScanOrder::RightThenLeft,
        ScanOrder::RightThenLeft => ScanOrder::LeftThenRight,
        ScanOrder::LeftThenRight => ScanOrder::InApp,
        ScanOrder::InApp => ScanOrder::Left,
    };

    data.is_connected = !data.is_connected;

    let mut c = data.logs.clone();
    c.push_str("Hello ");

    data.logs.push_str(&c);
}
