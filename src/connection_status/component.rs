use crate::{AppState, vars::SIZE_M, comms::COMMS};

use druid::{
    widget::{Either, Flex, Label},
    Color, Env, Widget, WidgetExt,
};

pub fn build_connection_status() -> impl Widget<AppState> {
    let either_dot = Either::new(
        |connected_to: &Option<String>, _: &Env| connected_to.is_some(),
        Label::new("")
            .fix_size(16., 16.)
            .background(Color::GREEN)
            .rounded(20.),
        Label::new("")
            .fix_size(16., 16.)
            .background(Color::RED)
            .rounded(20.),
    )
    .lens(AppState::connected_to);

    let template = Flex::row()
        .with_flex_child(either_dot, 1.) // dot
        .with_spacer(SIZE_M)
        .with_flex_child(
            Label::new(|connected_to: &Option<String>, _env: &Env| {
                if let Some(port_name) = connected_to {
                    format!("Connected To {}", port_name)
                } else {
                    "Disconnected".to_string()
                }
            })
            .lens(AppState::connected_to),
            1.,
        );

    template
}
