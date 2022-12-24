#![windows_subsystem = "windows"]

#![allow(deprecated)]
#![allow(unused)]

use druid::widget::{
    Checkbox, CrossAxisAlignment, Either, Flex, FlexParams, 
};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use trex_ui::actions::build_actions;
use trex_ui::comms::{commands, Channel, COMMS};
use trex_ui::connection_status::build_connection_status;
use trex_ui::controls::{build_controls, send_key};
use trex_ui::lights::build_lights;
use trex_ui::logs::build_logs;
use trex_ui::options::build_options;
use trex_ui::vars::{SIZE_L, SIZE_XXL};
use trex_ui::AppState;

fn main() -> Result<(), PlatformError> {
    
    let main_window = WindowDesc::new(ui_builder())
        .title("T-Rex® Control Panel")
        .window_size((750., 824.))
        .with_min_size((750., 500.));

    let mut data = AppState::default();
    data.counter = 0.5;

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();

    {
        let comms = COMMS.lock().unwrap();
        data.connected_to = Some(comms.port_name.clone());
    }

    // handle incoming events
    Channel::listen(move |cmd: String| {
        // println!("string: {:#?}", cmd);
        // println!("as bytes: {:#?}", cmd.as_bytes());
        // println!("pong: {:#?}", commands::PONG);
        // println!("is match: {:#?}", commands::PONG == cmd.as_bytes());

        event_sink.add_idle_callback(move |data: &mut AppState| {
            let as_bytes = cmd.as_bytes();

            match as_bytes {
                commands::PEDAL_SCAN => {
                    send_key(data);
                }
                commands::CALIBRATION_START => {
                    data.is_calibrating = true;
                }
                commands::CALIBRATION_PREVENTED
                | commands::CALIBRATION_STOP
                | commands::CALIBRATION_TIMEOUT => {
                    data.is_calibrating = false;
                }
                commands::CALIBRATION_IMMINENT => {}
                commands::ADJUST_START => {
                    data.is_auto_adjusting = true;
                }
                commands::ADJUST_STOP => {
                    data.is_auto_adjusting = false;
                }
                _ => (),
            }

            let log = format!("[EVENT] {}\n", cmd);
            data.logs.push_str(log.as_str());
        });
    });

    Ok(launcher
        // .log_to_console()
        .launch(data)
        .expect("Failed to launch the application"))
}

fn ui_builder() -> impl Widget<AppState> {
    let left = || {
        Flex::column()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .must_fill_main_axis(true)
            .with_child(
                Flex::row()
                    .with_flex_child(build_connection_status(), 1.)
                    .with_child(Checkbox::new("Show logs").lens(AppState::is_log_shown)),
            )
            .with_spacer(SIZE_XXL)
            .with_child(build_options())
            .with_spacer(SIZE_L)
            .with_child(build_actions())
            .with_spacer(SIZE_L)
            .with_child(build_lights())
            .with_spacer(SIZE_L)
            .with_child(build_controls())
    };

    let right = build_logs();

    let template = Either::new(
        |data, _| data.is_log_shown,
        Flex::row()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .with_flex_child(left(), 1.)
            .with_spacer(SIZE_XXL)
            .with_flex_child(right, FlexParams::new(1., CrossAxisAlignment::End)),
        Flex::row()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .with_flex_child(left(), 1.),
    )
    .padding(12.);

    template
}
