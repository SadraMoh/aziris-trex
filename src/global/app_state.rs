use druid::{Data, Lens};

use crate::options::{ScanMode, ScanOrder};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: f64,
    pub connected_to: Option<String>,
    pub logs: String,

    pub scan_order: ScanOrder,

    pub scan_mode: ScanMode,

    pub light_mix: f64,

    pub is_calibrating: bool,
    pub is_auto_adjusting: bool,

    pub is_log_shown: bool,

    pub is_options_shown: bool,
    pub is_kelvin_shown: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0.,
            connected_to: None,
            logs: String::new(),
            scan_order: ScanOrder::default(),
            scan_mode: ScanMode::default(),
            light_mix: 0.,
            is_calibrating: false,
            is_auto_adjusting: false,
            is_log_shown: false,
            is_options_shown: false,
            is_kelvin_shown: false,
        }
    }
}