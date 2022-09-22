use druid::{
    widget::{Button, Flex, Label, Padding, SizedBox},
    Color, Data, Lens, Widget, WidgetExt,
};

#[derive(Clone, Data, Lens)]
pub struct ConnectionStatus {
    is_connected: bool,
}

fn build_connection_status() -> impl Widget<ConnectionStatus> {
    let template = Flex::row()
        .with_flex_child(
            Label::new("HELLO").background(Color::RED),
            1.,
        )
        .with_flex_child(Label::new("Disconnected"), 1.);

    template
}
