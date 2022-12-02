/// VERBS MUST NOT EXCEED 32 BYTES

pub const PING: &[u8] = b"ping";
pub const PONG: &[u8] = b"pong";

pub const LASER_ON: &[u8] = b"laser_on";
pub const LASER_OFF: &[u8] = b"laser_off";

pub const LED_MIX_SCORCHING: &[u8] = b"led_scorch";
pub const LED_MIX_HOT: &[u8] = b"led_hot";
pub const LED_MIX_WARM: &[u8] = b"led_warm";
pub const LED_MIX_AUTO: &[u8] = b"led_auto";
pub const LED_MIX_COOL: &[u8] = b"led_cool";
pub const LED_MIX_COLD: &[u8] = b"led_cold";
pub const LED_MIX_FREEZING: &[u8] = b"led_freeze";

pub const PEDAL_SCAN: &[u8] = b"pedal_scan";

pub const CRADLE_UP: &[u8] = b"cradle_up";
pub const CRADLE_UP_STOP: &[u8] = b"cradle_up_stop";
pub const CRADLE_DOWN: &[u8] = b"cradle_down";
pub const CRADLE_DOWN_STOP: &[u8] = b"cradle_down_stop";

pub const CALIBRATION_START: &[u8] = b"calibration_start";
pub const CALIBRATION_STOP: &[u8] = b"calibration_stop";
pub const CALIBRATION_TIMEOUT: &[u8] = b"calibration_timeout";
pub const CALIBRATION_PREVENTED: &[u8] = b"calibration_prevented";
