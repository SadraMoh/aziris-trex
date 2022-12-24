use druid::{Color, WidgetId};
use once_cell::sync::Lazy;

pub const TEXTBOX_BACKGROUND: Color = Color::rgb8(28, 28, 28);
pub const BORDER_RADIUS: f64 = 6.0;

pub const SIZE_S: f64 = 5.;
pub const SIZE_M: f64 = 10.;
pub const SIZE_L: f64 = 15.;
pub const SIZE_XL: f64 = 20.;
pub const SIZE_XXL: f64 = 25.;

pub static ROOT_ID: Lazy<WidgetId> = Lazy::new(|| WidgetId::next());
