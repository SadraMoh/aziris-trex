
use druid::Data;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanOrder {
    Right,
    Left,
    RightThenLeft,
    LeftThenRight,
    InApp,
    InApp2,
}

impl Default for ScanOrder {
    fn default() -> Self {
        ScanOrder::InApp2
    }
}