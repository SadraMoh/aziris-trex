// mod comms;

// fn main() {
//     println!("{:#?}", available_ports().unwrap());

//     let mut channel = comms::Channel::init().unwrap();

//     loop {
//         let msg = b"ALPHA\n";
//         _ = channel.send(msg);
//     }

// }

use std::string;

use druid::widget::{Align, Button, Flex, Label, Padding, TextBox};
use druid::{
    AppLauncher, Data, Env, EventCtx, Lens, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    counter: u32,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    let data = AppState { counter: 0_u32 };

    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<AppState> {
    
    let button = Button::new("Increment").on_click(|_ctx, data: &mut AppState, _env| {
        data.counter += 1;
    });

    let label = Label::new(|data: &u32, _env: &_| format!("Counter says: {}", data))
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
