mod app_state;
mod comms;
mod connection_status;
mod logs;
mod options;
mod vars;

use crate::app_state::AppState;

// fn main() {
//     println!("{:#?}", available_ports().unwrap());

//     let mut channel = comms::Channel::init().unwrap();

//     loop {
//         let msg = b"ALPHA\n";
//         _ = channel.send(msg);
//     }

// }

use connection_status::build_connection_status;
use druid::widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label, Padding};
use druid::{AppLauncher, Color, Env, EventCtx, PlatformError, Widget, WidgetExt, WindowDesc};
use logs::build_logs;

use vars::SIZE_XXL;

use crate::options::{ScanMode, ScanOrder};
use options::build_options;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).window_size((900., 600.));

    let data = AppState {
        counter: 0_u32,
        is_connected: false,
        logs: String::new(),
        scan_mode: ScanMode::Auto,
        scan_order: ScanOrder::RightThenLeft,
    };

    Ok(AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application"))
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new("Increment").on_click(count_up);

    let template = Padding::new(
        12.,
        Flex::row()
            // left
            .with_flex_child(
                Flex::column()
                    .with_child(Label::new("Hello").center())
                    .with_child(Label::new("Hello").align_left())
                    .with_child(Label::new("Hello").align_right())
                    .with_child(build_connection_status().align_left())
                    // .with_child(
                    //     Label::new(|data: &AppState, _env: &Env| {
                    //         format!("Scan Order is: {:#?}", data.scan_order)
                    //     })
                    //     .align_right(),
                    // )
                    .with_child(build_options().align_left())
                    .border(Color::RED, 1.),
                FlexParams::new(1., CrossAxisAlignment::Start),
            )
            .with_spacer(SIZE_XXL)
            // right
            .with_flex_child(build_logs(), FlexParams::new(1., CrossAxisAlignment::End))
            // test button
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
    };

    data.is_connected = !data.is_connected;

    let mut c = data.logs.clone();
    c.push_str("Hello ");

    data.logs.push_str(&c);
}
