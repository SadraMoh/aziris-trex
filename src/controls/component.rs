use druid::{
  widget::{Button, Flex, Label, Padding},
  Env, EventCtx, Widget,
};

use crate::{
  vars::{SIZE_M, SIZE_S},
  AppState,
};

fn retract(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}
fn expand(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}

fn left(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}
fn right(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}

fn up(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}
fn down(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}

pub fn build_controls() -> impl Widget<AppState> {
  let template = Padding::new(
      SIZE_M,
      Flex::column()
          .with_flex_child(Label::new("Gradle controls"), 1.)
          .with_flex_child(
              Flex::row()
                  .with_flex_child(
                      Flex::column()
                          .with_flex_child(Button::new("→←").on_click(retract), 1.)
                          .with_spacer(SIZE_S)
                          .with_flex_child(Button::new("←→").on_click(expand), 1.),
                      1.,
                  )
                  .with_spacer(SIZE_M)
                  .with_flex_child(
                      Flex::row()
                          .with_flex_child(
                              Flex::column().with_flex_child(Button::new("←").on_click(left), 1.),
                              1.,
                          )
                          .with_flex_child(
                              Flex::column()
                                  .with_flex_child(Button::new("↑").on_click(up), 1.)
                                  .with_spacer(SIZE_S)
                                  .with_flex_child(Button::new("↓").on_click(down), 1.),
                              1.,
                          )
                          .with_flex_child(
                              Flex::column()
                                  .with_flex_child(Button::new("→").on_click(right), 1.),
                              1.,
                          ),
                      3.,
                  ),
              1.,
          ),
  );

  template
}
