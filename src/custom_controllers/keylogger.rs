use druid::KeyEvent;
use druid::widget::prelude::*;
use druid::widget::Controller;

use crate::vars::ROOT_ID;

pub struct KeyLogger<T> {
    keydown_fn: Box<dyn Fn(&KeyEvent, &mut EventCtx, &Event, &mut T, &Env)>,
    keyup_fn: Box<dyn Fn(&KeyEvent, &mut EventCtx, &Event, &mut T, &Env)>,
}

impl<T> KeyLogger<T> {
    pub fn new(
        keydown_fn: impl Fn(&KeyEvent, &mut EventCtx, &Event, &mut T, &Env) + 'static,
        keyup_fn: impl Fn(&KeyEvent, &mut EventCtx, &Event, &mut T, &Env) + 'static,
    ) -> Self {
        KeyLogger {
            keydown_fn: Box::new(keydown_fn),
            keyup_fn: Box::new(keyup_fn),
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for KeyLogger<T> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        
        ctx.set_focus(*ROOT_ID);
        
        match event {
            Event::KeyDown(key) => (self.keydown_fn)(key, ctx, event, data, env),
            Event::KeyUp(key) => (self.keyup_fn)(key, ctx, event, data, env),
            _ => (),
        }

        // Always pass on the event!
        child.event(ctx, event, data, env)
    }
}
