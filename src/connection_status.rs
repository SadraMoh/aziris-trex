use crate::app_state::AppState;

use druid::{
    widget::{Flex, Label},
    Color, Env, Widget, WidgetExt,
};

pub fn build_connection_status() -> impl Widget<AppState> {
    let template = Flex::row()
        .with_flex_child(
            Label::new(|data: &bool, _env: &Env| {
                if *data {
                    "Connected To"
                } else {
                    "Disconnected"
                }
            })
            .lens(AppState::is_connected)
            .background(Color::RED),
            1.,
        )
        .with_flex_child(Label::new("Disconnected"), 1.);

    template
}
