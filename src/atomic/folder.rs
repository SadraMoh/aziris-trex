use druid::{
    theme::BORDER_DARK,
    widget::{Button, Either, Flex, Label, Padding},
    Env, EventCtx, Widget, WidgetExt,
};

use crate::{
    vars::{BORDER_RADIUS, SIZE_M, SIZE_S},
    AppState,
};

pub fn folder(
    title: &str,
    child: impl Widget<AppState> + 'static,
    predicate: impl Fn(&AppState, &Env) -> bool + 'static,
    on_close: impl Fn(&mut EventCtx, &mut AppState, &Env) + 'static,
    on_open: impl Fn(&mut EventCtx, &mut AppState, &Env) + 'static,
) -> impl Widget<AppState> {
    let open = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        // head
        .with_child(
            Flex::row()
                .with_child(Label::new(title))
                .with_spacer(SIZE_S)
                .with_child(Button::from_label(Label::new("v")).on_click(on_open)),
        )
        .with_spacer(SIZE_M)
        // body
        .with_child(
            Padding::new(SIZE_M, child)
                .border(BORDER_DARK, 1.)
                .rounded(BORDER_RADIUS),
        );

    let closed = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        // head
        .with_child(
            Flex::row()
                .with_child(Label::new(title))
                .with_spacer(SIZE_S)
                .with_child(Button::from_label(Label::new(">")).on_click(on_close)),
        );

    Either::new(predicate, open, closed)
}
