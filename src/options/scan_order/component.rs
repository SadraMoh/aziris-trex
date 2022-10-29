use druid::{widget::RadioGroup, Widget, WidgetExt};

use crate::{options::ScanOrder, AppState};

pub fn build_scan_order() -> impl Widget<AppState> {
    const SCAN_ORDER_OPTIONS: [(&str, ScanOrder); 4] = [
        ("Left", ScanOrder::Left),
        ("Right", ScanOrder::Right),
        ("RightThenLeft", ScanOrder::RightThenLeft),
        ("LeftThenRight", ScanOrder::LeftThenRight),
    ];

    let radio_group = RadioGroup::new(SCAN_ORDER_OPTIONS.to_vec()).lens(AppState::scan_order);

    radio_group
}
