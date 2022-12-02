use druid::{Data, Lens};

use crate::options::{ScanMode, ScanOrder};

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub counter: f64,
    pub connected_to: Option<String>,
    pub logs: String,

    pub scan_order: ScanOrder,

    pub scan_mode: ScanMode,

    pub light_mix: f64,

    pub is_calibrating: bool,
}
