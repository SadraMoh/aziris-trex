use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub counter: u32,
}

impl AppState {}
