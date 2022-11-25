use druid::{
    widget::{Controller, Flex, Label, Slider},
    Color, Env, KeyOrValue, Widget, WidgetExt, UpdateCtx,
};

use crate::{atomic::group, vars::SIZE_S, AppState, comms::COMMS};

struct ValueChanged<T> {
    update_fn: Box<dyn Fn(&mut UpdateCtx, &T, &T, &Env)>,
}

impl<T> ValueChanged<T> {
    fn new(f: impl Fn(&mut UpdateCtx, &T, &T, &Env) + 'static) -> Self {
        ValueChanged { update_fn: Box::new(f) }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for ValueChanged<T> {
    
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &T,
        data: &T,
        env: &Env,
    ) {

        (self.update_fn)(ctx, old_data, data, env);

    }
}


pub fn build_lights() -> impl Widget<AppState> {
    // const SCAN_MODE_OPTIONS: [(&str, LightMix); 7] = [
    //     ("3500", LightMix::Scroching),
    //     ("4000", LightMix::Hot),
    //     ("4500", LightMix::Warm),
    //     ("Auto", LightMix::Auto),
    //     ("5500", LightMix::Cool),
    //     ("6000", LightMix::Cold),
    //     ("6500", LightMix::Freezing),
    // ];

    // let radio_group = Flex::column()
    //     .cross_axis_alignment(CrossAxisAlignment::Start)
    //     .must_fill_main_axis(true)
    //     .with_child(Label::new("Light Mix"))
    //     .with_spacer(SIZE_S)
    //     .with_child(
    //         RadioGroup::row(SCAN_MODE_OPTIONS.to_vec())
    //             .expand_width()
    //             .lens(AppState::light_mix),
    //     );

    // let template = group("Lights", radio_group);

    let slider = Slider::new()
        .with_range(-3., 3.)
        .track_color(KeyOrValue::Concrete(Color::RED))
        .axis(druid::widget::Axis::Horizontal)
        .with_step(1.)
        .expand_width()
        .controller(ValueChanged::new(|_,_, data,_| {
            let cmd: &[u8] = match *data as i64 {
                3 =>  b"led_scorch\0",
                2 =>  b"led_hot\0",
                1 =>  b"led_warm\0",
                0 =>  b"led_auto\0",
                -1 =>  b"led_cool\0",
                -2 =>  b"led_cold\0",
                -3 =>  b"led_freeze\0",
                _ => b"led_auto\0"
            };

            let mut comms = COMMS.lock().unwrap();
            comms.send(cmd).unwrap();

        }))
        .lens(AppState::light_mix);

    let template = group(
        "Lights",
        Flex::row()
            .must_fill_main_axis(true)
            .with_child(
                Label::new(|data: &AppState, _env: &Env| match data.light_mix {
                    x if x == 0. => "Auto".to_string(),
                    i => format!("{}", 5000. + (i * 500.)),
                })
                .fix_width(32.),
            )
            .with_spacer(SIZE_S)
            .with_flex_child(slider, 1.),
    );

    template
}
