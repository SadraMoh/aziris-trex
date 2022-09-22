use druid::{
    widget::{Flex, Label},
    Color, Data, Env, Lens, Widget, WidgetExt,
};

#[derive(Clone, Data, Lens)]
pub struct ConnectionStatus {
    is_connected: bool,
}

fn build_connection_status() -> impl Widget<ConnectionStatus> {
    let template = Flex::row()
        .with_flex_child(
            Label::new(|data: &bool, _env: &Env| {
                if *data {
                    "Connected To"
                } else {
                    "Disconnected"
                }
            })
            .lens(ConnectionStatus::is_connected)
            .background(Color::RED),
            1.,
        )
        .with_flex_child(Label::new("Disconnected"), 1.);

    template
}
