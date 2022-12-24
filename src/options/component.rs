use druid::{widget::Flex, Widget, WidgetExt};

use crate::{
    atomic::{folder},
    options::{build_scan_mode, build_scan_order},
    vars::SIZE_M,
    AppState,
};

pub fn build_options() -> impl Widget<AppState> {
    let template = folder(
        "Options",
        Flex::row()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .must_fill_main_axis(true)
            .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
            .with_flex_child(build_scan_order().expand_width().align_left(), 1.)
            .with_spacer(SIZE_M)
            .with_flex_child(build_scan_mode().expand_width().align_left(), 1.),
        |data: &AppState, _:_| { data.is_options_shown },
        |_:_ , data, _:_| { data.is_options_shown = !data.is_options_shown },
        |_:_ , data, _:_| { data.is_options_shown = !data.is_options_shown },
    );

    template
}
