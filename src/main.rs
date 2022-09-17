// mod comms;

// fn main() {
//     println!("{:#?}", available_ports().unwrap());

//     let mut channel = comms::Channel::init().unwrap();

//     loop {
//         let msg = b"ALPHA\n";
//         _ = channel.send(msg);
//     }

// }

use druid::text::Formatter;
use druid::widget::{Align, Button, Flex, Label, Padding, TextBox};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Env, EventCtx};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window).launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    // // The label text will be computed dynamically based on the current locale and count
    // let text =
    //     LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    // let label = Label::new(text).padding(5.0).center();
    // let button = Button::new("increment")
    //     .on_click(|_ctx, data, _env| *data += 1)
    //     .padding(5.0);

    // Flex::column().with_child(label).with_child(button)

    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());

    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new(text), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0)
                    .with_flex_child(
                        Button::new("increment")
                            .on_click(|_ctx, data, _env| {
                                *data += 1;
                            })
                            .padding(5.0),
                        1.0,
                    ),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom right")), 1.0),
                1.0,
            ),
    )
}