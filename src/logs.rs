use std::sync::Arc;

use crate::{
    app_state::AppState,
    vars::{BORDER_RADIUS, SIZE_S},
    vars::{SIZE_L, TEXTBOX_BACKGROUND},
};

use druid::{
    widget::{Flex, Label, Padding},
    Color, Env, RoundedRectRadii, Widget, WidgetExt,
};

pub fn build_logs() -> impl Widget<AppState> {
    let template = Flex::column()
        .with_child(Label::new("Logs").align_left())
        .with_spacer(SIZE_S)
        .with_flex_child(
            Padding::new(
                SIZE_L,
                Label::new(|data: &Arc<Vec<String>>, _env: &Env| data.join("\n"))
                    .lens(AppState::logs)
                    .expand(),
            )
            .background(TEXTBOX_BACKGROUND)
            .rounded(RoundedRectRadii::from_single_radius(BORDER_RADIUS)),
            1.,
        )
        .border(Color::GREEN, 1.);

    template
}
