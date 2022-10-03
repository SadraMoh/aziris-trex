mod scan_mode;
mod scan_order;

use druid::{
    widget::{Align, Flex, Label},
    Color, UnitPoint, Widget, WidgetExt,
};

pub use scan_mode::{build_scan_mode, ScanMode};
pub use scan_order::{build_scan_order, ScanOrder};

use crate::{
    app_state::AppState,
    vars::{BORDER_RADIUS, BORDER_THICKNESS, SIZE_L, SIZE_M, SIZE_S, TEXTBOX_BACKGROUND},
};
pub fn build_options() -> impl Widget<AppState> {
    let template = Flex::column()
        .with_child(Label::new("Options").align_left())
        .with_spacer(SIZE_M)
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    Flex::column()
                        .with_child(Label::new("Scan order").align_left())
                        .with_spacer(SIZE_S)
                        .with_child(build_scan_order().align_left()),
                    1.,
                )
                .with_flex_child(
                    Flex::column()
                        .with_child(Label::new("Scan mode").align_left())
                        .with_spacer(SIZE_S)
                        .with_child(build_scan_mode().align_left()),
                    1.,
                )
                .padding(SIZE_L)
                .border(TEXTBOX_BACKGROUND, BORDER_THICKNESS)
                .rounded(BORDER_RADIUS),
            1.,
        );

    template
}
