use druid::{
    widget::{Flex, Label, Padding, RadioGroup},
    Widget, WidgetExt,
};

use crate::{options::ScanOrder, vars::SIZE_M, AppState};

pub fn build_options() -> impl Widget<AppState> {
    const SCAN_ORDER_OPTIONS: [(&str, ScanOrder); 4] = [
        ("Left", ScanOrder::Left),
        ("Right", ScanOrder::Right),
        ("RightThenLeft", ScanOrder::RightThenLeft),
        ("LeftThenRight", ScanOrder::LeftThenRight),
    ];

    let radio_group = RadioGroup::new(SCAN_ORDER_OPTIONS.to_vec()).lens(AppState::scan_order);

    let template = Padding::new(
        SIZE_M,
        Flex::column()
            .with_flex_child(Label::new("Scan orer"), 1.)
            .with_child(radio_group),
    );

    template
}
