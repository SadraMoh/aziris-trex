use std::sync::Arc;

use druid::{Data, Lens};

use crate::options::{ScanMode, ScanOrder};

pub const MAX_LOG_LINE_COUNT: usize = 256;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: u32,
    pub is_connected: bool,
    pub logs: Arc<Vec<String>>,

    pub scan_order: ScanOrder,

    pub scan_mode: ScanMode,
}
