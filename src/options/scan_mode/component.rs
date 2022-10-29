use druid::{widget::RadioGroup, Widget, WidgetExt};

use crate::{AppState, options::ScanMode};

pub fn build_scan_mode() -> impl Widget<AppState> {
  const SCAN_MODE_OPTIONS: [(&str, ScanMode); 3] = [
      ("Pedal", ScanMode::Pedal),
      ("Panel", ScanMode::Panel),
      ("Auto", ScanMode::Auto),
  ];

  let radio_group = RadioGroup::new(SCAN_MODE_OPTIONS.to_vec()).lens(AppState::scan_mode);

  radio_group
}
