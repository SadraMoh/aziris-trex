use druid::{widget::RadioGroup, Data, Widget, WidgetExt};

use crate::app_state::AppState;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanMode {
    Pedal,
    Panel,
    Auto,
}

pub fn build_scan_mode() -> impl Widget<AppState> {
    const SCAN_MODE_OPTIONS: [(&str, ScanMode); 3] = [
        ("Pedal", ScanMode::Pedal),
        ("Panel", ScanMode::Panel),
        ("Auto", ScanMode::Auto),
    ];

    let radio_group = RadioGroup::column(SCAN_MODE_OPTIONS.to_vec()).lens(AppState::scan_mode);

    radio_group
}
