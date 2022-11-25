use druid::{Data};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanMode {
    Pedal,
    Panel,
    Auto,
}

impl Default for ScanMode {
    fn default() -> Self {
        ScanMode::Pedal
    }
}
