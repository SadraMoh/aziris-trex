use druid::{
    widget::{Controller, Flex, Label, Slider},
    Color, Env, KeyOrValue, Widget, WidgetExt, UpdateCtx,
};

use crate::{atomic::group, vars::SIZE_S, AppState, comms::{COMMS, commands}};

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
                3 =>  commands::LED_MIX_SCORCHING,
                2 =>  commands::LED_MIX_HOT,
                1 =>  commands::LED_MIX_WARM,
                0 =>  commands::LED_MIX_AUTO,
                -1 =>  commands::LED_MIX_COOL,
                -2 =>  commands::LED_MIX_COLD,
                -3 =>  commands::LED_MIX_FREEZING,
                _ => commands::LED_MIX_AUTO
            };

            let mut comms = COMMS.lock().unwrap();
            comms.cmd(cmd).unwrap();

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
