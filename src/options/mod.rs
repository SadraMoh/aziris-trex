mod scan_mode;
mod scan_order;

use druid::{
    widget::{Flex, Label},
    Color, Widget, WidgetExt,
};

pub use scan_mode::{build_scan_mode, ScanMode};
pub use scan_order::{build_scan_order, ScanOrder};

use crate::{
    app_state::AppState,
    vars::{BORDER_RADIUS, BORDER_THICKNESS, SIZE_M, SIZE_S, TEXTBOX_BACKGROUND},
};
pub fn build_options() -> impl Widget<AppState> {
    let template = Flex::column()
        .with_child(Label::new("Options"))
        .with_spacer(SIZE_S)
        .with_child(
            Flex::row()
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(Label::new("Scan orer").align_left(), 1.)
                        .with_child(build_scan_order()),
                    1.,
                )
                .with_spacer(SIZE_M)
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(Label::new("Scan orer").align_left(), 1.)
                        .with_child(build_scan_mode()),
                    1.,
                )
                .expand_width(),
        )
        .padding(SIZE_M)
        .border(TEXTBOX_BACKGROUND, BORDER_THICKNESS)
        .rounded(BORDER_RADIUS);

    template
}
