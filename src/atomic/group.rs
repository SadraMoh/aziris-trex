use druid::{
    theme::BORDER_DARK,
    widget::{Flex, Label, Padding},
    Widget, WidgetExt,
};

use crate::{
    vars::{BORDER_RADIUS, SIZE_M},
    AppState,
};

pub fn group(title: &str, child: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
    let template = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(Label::new(title))
        .with_spacer(SIZE_M)
        .with_child(
            Padding::new(SIZE_M, child)
                .border(BORDER_DARK, 1.)
                .rounded(BORDER_RADIUS),
        );

    template
}
