/// VERBS MUST NOT EXCEED 32 BYTES

pub const PING: &[u8] = b"ping";
pub const PONG: &[u8] = b"pong";

pub const MODE_PEDAL: &[u8] = b"mode_pedal";
pub const MODE_PANEL: &[u8] = b"mode_panel";
pub const MODE_AUTO: &[u8] = b"mode_auto";

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

pub const CRADLE_CLOSE: &[u8] = b"cradle_close";
pub const CRADLE_CLOSE_STOP: &[u8] = b"cradle_close_stop";
pub const CRADLE_OPEN: &[u8] = b"cradle_open";
pub const CRADLE_OPEN_STOP: &[u8] = b"cradle_open_stop";
pub const CRADLE_LEFT: &[u8] = b"cradle_left";
pub const CRADLE_LEFT_STOP: &[u8] = b"cradle_left_stop";
pub const CRADLE_RIGHT: &[u8] = b"cradle_right";
pub const CRADLE_RIGHT_STOP: &[u8] = b"cradle_right_stop";

pub const CALIBRATION_START: &[u8] = b"calibration_start";
pub const CALIBRATION_STOP: &[u8] = b"calibration_stop";
pub const CALIBRATION_IMMINENT: &[u8] = b"calibration_imminent";
pub const CALIBRATION_TIMEOUT: &[u8] = b"calibration_timeout";
pub const CALIBRATION_PREVENTED: &[u8] = b"calibration_prevented";

pub const ADJUST_START: &[u8] = b"adjust_start";
pub const ADJUST_STOP: &[u8] = b"adjust_stop";