use crate::app_state::AppState;

use druid::{
    widget::{Either, Flex, Label},
    Color, Env, Widget, WidgetExt,
};

pub fn build_connection_status() -> impl Widget<AppState> {
    let either_dot = Either::new(
        |is_connected: &bool, _: &Env| *is_connected,
        Label::new("")
            .fix_size(16., 16.)
            .background(Color::GREEN)
            .rounded(20.),
        Label::new("")
            .fix_size(16., 16.)
            .background(Color::RED)
            .rounded(20.),
    )
    .lens(AppState::is_connected);

    let template = Flex::row()
        .with_flex_child(either_dot, 1.) // dot
        .with_flex_child(
            Label::new(|is_connected: &bool, _env: &Env| {
                if *is_connected {
                    "Connected To"
                } else {
                    "Disconnected"
                }
            })
            .lens(AppState::is_connected),
            1.,
        );

    template
}
