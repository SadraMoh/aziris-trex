use druid::{Data};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanMode {
    Pedal,
    Panel,
    Auto,
}
