use druid::{widget::RadioGroup, Data, Widget, WidgetExt};

use crate::app_state::AppState;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanOrder {
    Right,
    Left,
    RightThenLeft,
    LeftThenRight,
}

pub fn build_scan_order() -> impl Widget<AppState> {
    const SCAN_ORDER_OPTIONS: [(&str, ScanOrder); 4] = [
        ("Left", ScanOrder::Left),
        ("Right", ScanOrder::Right),
        ("RightThenLeft", ScanOrder::RightThenLeft),
        ("LeftThenRight", ScanOrder::LeftThenRight),
    ];

    let radio_group = RadioGroup::column(SCAN_ORDER_OPTIONS.to_vec()).lens(AppState::scan_order);

    radio_group
}
