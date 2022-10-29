use druid::widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label, Padding};
use druid::{AppLauncher, Color, Env, EventCtx, PlatformError, Widget, WidgetExt, WindowDesc};
use trex_ui::actions::build_actions;
use trex_ui::connection_status::build_connection_status;
use trex_ui::controls::build_controls;
use trex_ui::logs::build_logs;
use trex_ui::options::{build_options, ScanMode, ScanOrder};
use trex_ui::vars::{SIZE_L, SIZE_XXL};
use trex_ui::AppState;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).window_size((900., 600.));

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

    let source = vec![("Hello", ScanMode::Auto), ("World", ScanMode::Panel)];
    let radios = druid::widget::RadioGroup::new(source).lens(AppState::scan_mode);

    let template = Padding::new(
        12.,
        Flex::row()
            // left
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
                    .must_fill_main_axis(true)
                    .with_child(Label::new("Hello").center())
                    .with_child(Label::new("Hello").align_left())
                    .with_child(Label::new("Hello").align_right())
                    .with_child(build_connection_status())
                    .with_spacer(SIZE_L)
                    .with_child(build_options())
                    .with_spacer(SIZE_L)
                    .with_child(build_actions())
                    .with_spacer(SIZE_L)
                    .with_child(build_controls())
                    .border(Color::RED, 1.),
                1.
            )
            .with_spacer(SIZE_XXL)
            // right
            .with_flex_child(build_logs(), FlexParams::new(1., CrossAxisAlignment::End))
            // test button
            .with_child(button)
            .with_child(radios)
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
