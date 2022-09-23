use core::fmt;

use druid::{
    widget::{Axis, Button, Flex, Label, Padding, Radio, RadioGroup},
    Env, Widget, WidgetExt, Data,
};

use crate::{
    app_state::{self, AppState},
    vars::SIZE_M,
};

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanOrder {
    Right,
    Left,
    RightThenLeft,
    LeftThenRight,
}

impl fmt::Display for ScanOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Data, PartialEq)]
pub enum ScanMode {
    Pedal,
    Panel,
    Auto,
}
impl fmt::Display for ScanMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn build_options() -> impl Widget<AppState> {
    const SCAN_ORDER_OPTIONS: [(&str, ScanOrder); 4] = [
        ("Left", ScanOrder::Left),
        ("Right", ScanOrder::Right),
        ("RightThenLeft", ScanOrder::RightThenLeft),
        ("LeftThenRight", ScanOrder::LeftThenRight),
    ];

    let radio_group = RadioGroup::column(SCAN_ORDER_OPTIONS.to_vec()).lens(AppState::scan_order);

    let template = Padding::new(
        SIZE_M,
        Flex::column()
            .with_flex_child(Label::new("Scan orer"), 1.)
            .with_child(radio_group),
    );

    template
}
