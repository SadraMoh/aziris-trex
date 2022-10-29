use druid::{Data, Lens};

use crate::options::{ScanMode, ScanOrder};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: u32,
    pub is_connected: bool,
    pub logs: String,

    pub scan_order: ScanOrder,

    pub scan_mode: ScanMode,
}
