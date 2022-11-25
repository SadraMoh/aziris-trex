use druid::{
    widget::{CrossAxisAlignment, Flex, Label, RadioGroup},
    Widget, WidgetExt,
};

use crate::{options::ScanMode, vars::SIZE_S, AppState};

pub fn build_scan_mode() -> impl Widget<AppState> {
    const SCAN_MODE_OPTIONS: [(&str, ScanMode); 3] = [
        ("Pedal", ScanMode::Pedal),
        ("Panel", ScanMode::Panel),
        ("Auto", ScanMode::Auto),
    ];

    let radio_group = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(Label::new("Scan order"))
        .with_spacer(SIZE_S)
        .with_child(RadioGroup::column(SCAN_MODE_OPTIONS.to_vec()).lens(AppState::scan_mode));

    radio_group
}
