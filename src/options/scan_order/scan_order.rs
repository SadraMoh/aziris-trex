use std::default;

use druid::Data;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanOrder {
    Right,
    Left,
    RightThenLeft,
    LeftThenRight,
    InApp,
}

impl Default for ScanOrder {
    fn default() -> Self {
        ScanOrder::Right
    }
}