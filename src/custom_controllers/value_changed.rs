use druid::{UpdateCtx, Env, widget::Controller, Widget};

pub struct ValueChanged<T> {
    update_fn: Box<dyn Fn(&mut UpdateCtx, &T, &T, &Env)>,
}

impl<T> ValueChanged<T> {
    pub fn new(f: impl Fn(&mut UpdateCtx, &T, &T, &Env) + 'static) -> Self {
        ValueChanged { update_fn: Box::new(f) }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for ValueChanged<T> {
    
    fn update(
        &mut self,
        _child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &T,
        data: &T,
        env: &Env,
    ) {

        (self.update_fn)(ctx, old_data, data, env);

    }
}