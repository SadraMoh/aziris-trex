use druid::{Data};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanMode {
    Auto, 
    Panel,
    Pedal,
}

impl Default for ScanMode {
    fn default() -> Self {
        ScanMode::Auto
    }
}
