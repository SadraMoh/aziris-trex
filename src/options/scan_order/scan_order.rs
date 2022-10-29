use druid::Data;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanOrder {
    Right,
    Left,
    RightThenLeft,
    LeftThenRight,
}
