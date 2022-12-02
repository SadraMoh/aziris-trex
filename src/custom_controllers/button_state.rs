use druid::{widget::Controller, Env,  Widget};

/// Handle mousedown and mouseup for a widget
pub struct ButtonState<T> {
    mousedown_fn: Box<dyn Fn(&mut druid::EventCtx, &druid::Event,  &mut T,  &Env)>,
    mouseup_fn: Box<dyn Fn(&mut druid::EventCtx, &druid::Event,  &mut T,  &Env)>,
}

impl<T> ButtonState<T> {
    pub fn new(
        mousedown: impl Fn(&mut druid::EventCtx, &druid::Event,  &mut T,  &Env) + 'static,
        mouseup: impl Fn(&mut druid::EventCtx, &druid::Event,  &mut T,  &Env) + 'static,
    ) -> Self {
        ButtonState {
            mousedown_fn: Box::new(mousedown),
            mouseup_fn: Box::new(mouseup),
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for ButtonState<T> {
    fn event(&mut self, _child: &mut W, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &Env) {
        match event {
            druid::Event::MouseDown(_) => {
                (self.mousedown_fn)(ctx, event, data, env);
            },
            druid::Event::MouseUp(_) => {
                (self.mouseup_fn)(ctx, event, data, env);
            },
            _ => (),
        }
    }
}
