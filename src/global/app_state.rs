use druid::{Data, Lens};

use crate::options::{ScanMode, ScanOrder};

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    pub counter: u32,
    pub is_connected: bool,
    pub logs: String,

    pub scan_order: ScanOrder,

    pub scan_mode: ScanMode,

    pub light_mix: f64,
}
