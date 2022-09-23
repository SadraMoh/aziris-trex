mod app_state;
mod connection_status;
mod logs;
mod options;
mod vars;

use std::sync::Arc;

use crate::app_state::AppState;

// mod comms;

// fn main() {
//     println!("{:#?}", available_ports().unwrap());

//     let mut channel = comms::Channel::init().unwrap();

//     loop {
//         let msg = b"ALPHA\n";
//         _ = channel.send(msg);
//     }

// }

use druid::widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label, Padding, Scroll};
use druid::{AppLauncher, Color, Env, EventCtx, PlatformError, Widget, WidgetExt, WindowDesc};
use logs::build_logs;
use options::{build_options, ScanMode, ScanOrder};
use vars::{SIZE_XL, SIZE_XXL};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).window_size((900., 600.));

    let data = AppState {
        counter: 0_u32,
        is_connected: false,
        logs: Arc::new(Vec::new()),
        scan_mode: ScanMode::Auto,
        scan_order: ScanOrder::RightThenLeft,
    };

    Ok(AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application"))
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new("Increment").on_click(count_up);

    let label = Label::new(|data: &u32, _env: &Env| format!("Counter says: {}", data))
        .lens(AppState::counter);

    let template = Padding::new(
        12.,
        Flex::row()
            // left
            .with_flex_child(
                Flex::column()
                    .with_child(Label::new("Hello").center())
                    .with_child(Label::new("Hello").align_left())
                    .with_child(Label::new("Hello").align_right())
                    .with_child(
                        Scroll::new(Label::new(|data: &AppState, _env: &Env| {
                            format!("Scan Order is: {:#?}", data.scan_order)
                        }))
                        .vertical()
                        .align_right(),
                    )
                    .with_flex_child(
                        build_options(),
                        FlexParams::new(1., CrossAxisAlignment::End),
                    )
                    .border(Color::RED, 1.),
                FlexParams::new(1., CrossAxisAlignment::Start),
            )
            .with_spacer(SIZE_XXL)
            // right
            .with_flex_child(build_logs(), FlexParams::new(1., CrossAxisAlignment::End))
            // test
            .with_child(button)
            .border(Color::BLUE, 1.),
    );

    template
}

fn count_up(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.counter += 1;
    data.scan_order = match data.scan_order {
        ScanOrder::Left => ScanOrder::Right,
        ScanOrder::Right => ScanOrder::RightThenLeft,
        ScanOrder::RightThenLeft => ScanOrder::LeftThenRight,
        ScanOrder::LeftThenRight => ScanOrder::Left,
    }
}
