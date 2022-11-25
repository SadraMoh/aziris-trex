#[derive(Clone, PartialEq)]
pub enum CMD {
    Ping = 1,
    Pong = 2,
    LightSet3500 = 3,
    LightSet4000 = 4,
    LightSet4500 = 5,
    LightSet5000 = 6,
    LightSet5500 = 7,
    LightSet6000 = 8,
    LightSet6500 = 9,
}