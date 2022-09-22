mod app_state;
mod connection_status;

use app_state::AppState;

// mod comms;

// fn main() {
//     println!("{:#?}", available_ports().unwrap());

//     let mut channel = comms::Channel::init().unwrap();

//     loop {
//         let msg = b"ALPHA\n";
//         _ = channel.send(msg);
//     }

// }

use druid::widget::{Align, Button, Flex, Label, Padding, FlexParams, CrossAxisAlignment};
use druid::{
    AppLauncher, Color, Env, EventCtx, PlatformError, UnitPoint, Widget, WidgetExt, WindowDesc,
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).window_size((900., 600.));

    let data = AppState { counter: 0_u32 };

    Ok(AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application"))
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new("Increment").on_click(count_up);

    let label = Label::new(|data: &u32, _env: &Env| format!("Counter says: {}", data))
        .lens(AppState::counter);

    let template = Padding::new(
        10.,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_child(Label::new("Hello").center())
                    .with_child(Label::new("Hello").align_left())
                    .with_child(Label::new("Hello").align_right())
                    .border(Color::RED, 1.),
                    FlexParams::new(1., CrossAxisAlignment::Start),
            )
            .with_flex_child(
                Flex::column()
                    .with_child(Label::new("Hello").center())
                    .with_child(Label::new("Hello").align_left())
                    .with_child(Label::new("Hello").align_right())
                    .border(Color::RED, 1.),
                    FlexParams::new(1., CrossAxisAlignment::End),
            )
            .border(Color::BLUE, 1.),
    );

    template
}

fn count_up(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.counter += 1;
}
