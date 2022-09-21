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

use druid::widget::{Button, Flex, Label, Padding};
use druid::{AppLauncher, Env, EventCtx, PlatformError, Widget, WidgetExt, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    let data = AppState { counter: 0_u32 };

    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new("Increment").on_click(count_up);

    let label = Label::new(|data: &u32, _env: &Env| format!("Counter says: {}", data))
        .lens(AppState::counter);

    let template = Padding::new(
        10.,
        Flex::row().with_flex_child(
            Flex::column()
                .with_flex_child(button, 1.)
                .with_flex_child(label, 1.),
            1.,
        ),
    );

    template
}

fn count_up(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.counter += 1;
}
