use druid::widget::{CrossAxisAlignment, Flex, FlexParams, Padding};
use druid::{AppLauncher, Env, EventCtx, PlatformError, Widget, WindowDesc};
use trex_ui::actions::build_actions;
use trex_ui::connection_status::build_connection_status;
use trex_ui::controls::build_controls;
use trex_ui::logs::build_logs;
use trex_ui::options::{build_options, ScanMode, ScanOrder};
use trex_ui::vars::{SIZE_L, SIZE_XXL};
use trex_ui::AppState;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .title("T-Rex Control Panel")
        .window_size((900., 540.))
        .with_min_size((640., 540.));

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
    let template = Padding::new(
        12.,
        Flex::row()
            // left
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
                    .with_child(build_controls()), // test button
                // .with_child(button)
                // .with_child(radios)
                1.,
            )
            .with_spacer(SIZE_XXL)
            // right
            .with_flex_child(build_logs(), FlexParams::new(1., CrossAxisAlignment::End)),
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
