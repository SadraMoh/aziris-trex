use crate::{
    app_state::AppState,
    vars::{BORDER_RADIUS, SIZE_S},
    vars::{TEXTBOX_BACKGROUND},
};

use druid::{
    widget::{Flex, Label, TextBox},
    Color, RoundedRectRadii, Widget, WidgetExt,
};

pub fn build_logs() -> impl Widget<AppState> {

    // let viewer = Label::new(|data: &String, _env: &_| data.clone())
    //     .lens(AppState::logs)
    //     .background(TEXTBOX_BACKGROUND)
    //     .rounded(RoundedRectRadii::from_single_radius(BORDER_RADIUS))
    //     .expand_height()
    //     .fix_width(50.)
    //     .scroll()
    //     .vertical()
    //     .expand_width();

    let viewer = TextBox::multiline()
        .lens(AppState::logs)
        .background(TEXTBOX_BACKGROUND)
        .rounded(RoundedRectRadii::from_single_radius(BORDER_RADIUS))
        .expand();

    let template = Flex::column()
        .with_child(Label::new("Logs").align_left())
        .with_spacer(SIZE_S)
        .with_flex_child(viewer, 1.)
        .border(Color::GREEN, 1.);

    template
}
