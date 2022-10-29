use druid::{
  widget::{Button, Flex, Label, Padding},
  Env, EventCtx, Widget,
};

use crate::{
  AppState,
  vars::{SIZE_M, SIZE_S},
};

fn auto_adjust(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}
fn calibrate(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {}

pub fn build_actions() -> impl Widget<AppState> {
  let template = Padding::new(
      SIZE_M,
      Flex::column()
          .with_flex_child(Label::new("Actions"), 1.)
          .with_flex_child(
              Flex::row()
                  // .with_flex_child(Button::new("Automatic adjustment"), 1.)
                  // .with_flex_child(Button::new("Calibrate"), 1.),
                  .with_child(Button::new("Automatic adjustment").on_click(auto_adjust))
                  .with_spacer(SIZE_S)
                  .with_child(Button::new("Calibrate").on_click(calibrate)),
              1.,
          ),
  );

  template
}
