use druid::{
    widget::{Flex, Label, RadioGroup, CrossAxisAlignment},
    Widget, WidgetExt,
};

use crate::{options::ScanOrder, vars::SIZE_S, AppState};

pub fn build_scan_order() -> impl Widget<AppState> {
    const SCAN_ORDER_OPTIONS: [(&str, ScanOrder); 5] = [
        ("Left", ScanOrder::Left),
        ("Right", ScanOrder::Right),
        ("RightThenLeft", ScanOrder::RightThenLeft),
        ("LeftThenRight", ScanOrder::LeftThenRight),
        ("InApp", ScanOrder::InApp),
    ];

    let radio_group = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(Label::new("Scan order"))
        .with_spacer(SIZE_S)
        .with_child(RadioGroup::new(SCAN_ORDER_OPTIONS.to_vec()).lens(AppState::scan_order));

    radio_group
}
