use druid::{
    widget::{CrossAxisAlignment, Flex, Label, RadioGroup},
    Widget, WidgetExt,
};

use crate::{custom_controllers::ValueChanged, options::ScanMode, vars::SIZE_S, AppState, comms::{COMMS, commands}};

pub fn build_scan_mode() -> impl Widget<AppState> {
    const SCAN_MODE_OPTIONS: [(&str, ScanMode); 3] = [
        ("Auto", ScanMode::Auto),
        ("Panel", ScanMode::Panel),
        ("Pedal", ScanMode::Pedal),
    ];

    let radio_group = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(Label::new("Scan mode"))
        .with_spacer(SIZE_S)
        .with_child(
            RadioGroup::column(SCAN_MODE_OPTIONS.to_vec())
                .controller(ValueChanged::new(|_, _, data, _| {
                    let mut comms = COMMS.lock().unwrap();
                    (match data {
                        ScanMode::Auto => comms.cmd(commands::MODE_AUTO),
                        ScanMode::Panel => comms.cmd(commands::MODE_PANEL),
                        ScanMode::Pedal => comms.cmd(commands::MODE_PEDAL),
                    }).unwrap();
                }))
                .lens(AppState::scan_mode),
        );

    radio_group
}
