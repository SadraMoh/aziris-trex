use druid::{
    widget::{Button, CrossAxisAlignment, Flex, FlexParams},
    Env, EventCtx, Widget, WidgetExt,
};

use crate::{
    atomic::group,
    vars::{SIZE_S, SIZE_XL},
    AppState,
};

fn retract(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn expand(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

fn left(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn right(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

fn up(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}
fn down(_ctx: &mut EventCtx, _data: &mut AppState, _env: &Env) {}

const BUTTON_SIZE: f64 = 64.;

pub fn build_controls() -> impl Widget<AppState> {
    let template = group(
        "Controls",
        Flex::row()
            .must_fill_main_axis(true)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Button::new("→←").on_click(retract).expand(), 1.)
                    .with_spacer(SIZE_S)
                    .with_flex_child(Button::new("←→").on_click(expand).expand(), 1.),
                1.,
            )
            .with_spacer(SIZE_XL)
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        Button::new("←").on_click(left).expand(),
                        FlexParams::new(1., CrossAxisAlignment::End),
                    )
                    .with_spacer(SIZE_S)
                    .with_flex_child(
                        Flex::column()
                            .with_flex_child(Button::new("↑").on_click(up).expand(), 1.)
                            .with_spacer(SIZE_S)
                            .with_flex_child(Button::new("↓").on_click(down).expand(), 1.),
                        1.,
                    )
                    .with_spacer(SIZE_S)
                    .with_flex_child(
                        Button::new("→").on_click(right).expand(),
                        FlexParams::new(1., CrossAxisAlignment::End),
                    ),
                3.,
            )
            .fix_height(BUTTON_SIZE),
    );

    template
}
