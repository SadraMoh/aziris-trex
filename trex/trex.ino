#include "queue.h"
#include "state_button.h"

#pragma region COMMANDS

#define CMD_PING "ping"
#define CMD_PONG "pong"

#define CMD_LASER_ON "laser_on"
#define CMD_LASER_OFF "laser_off"

#define CMD_LED_MIX_SCORCHING "led_scorch"
#define CMD_LED_MIX_HOT "led_hot"
#define CMD_LED_MIX_WARM "led_warm"
#define CMD_LED_MIX_AUTO "led_auto"
#define CMD_LED_MIX_COOL "led_cool"
#define CMD_LED_MIX_COLD "led_cold"
#define CMD_LED_MIX_FREEZING "led_freeze"

#define CMD_PEDAL_SCAN "pedal_scan"

#pragma endregion COMMANDS

#pragma region CRADLE
const unsigned short CRADLE_RIGHT_CLOSE_MOTOR = 2;
const unsigned short CRADLE_RIGHT_OPEN_MOTOR = 3;
const unsigned short CRADLE_LEFT_CLOSE_MOTOR = 4;
const unsigned short CRADLE_LEFT_OPEN_MOTOR = 5;

const unsigned short CRADLE_LEFT_OPEN_SENSOR = 22;
const unsigned short CRADLE_LEFT_CLOSE_SENSOR = 23;
const unsigned short CRADLE_RIGHT_OPEN_SENSOR = 24;
const unsigned short CRADLE_RIGHT_CLOSE_SENSOR = 25;

const unsigned short CRADLE_UP_MOTOR = 33;
const unsigned short CRADLE_DOWN_MOTOR = 35;
const unsigned short CRADLE_UP_SENSOR = 32;
const unsigned short CRADLE_DOWN_SENSOR = 34;
#pragma endregion CRADLE

#pragma region GLASS
const unsigned short GLASS_UP_MOTOR = 31;
const unsigned short GLASS_DOWN_MOTOR = 29;
const unsigned short GLASS_UP_SENSOR = 28;
const unsigned short GLASS_DOWN_SENSOR = 30;

const unsigned short GLASS_AUTOLEVEL_SENSOR = 36;
#pragma endregion GLASS

#pragma region MISC
const unsigned short LED_WHITE = 7;
const unsigned short LED_YELLOW = 6;

const unsigned short LASER = 37;

const unsigned short PEDAL = 26;

const unsigned short CAMERA_LEFT_POWER = 38;
const unsigned short CAMERA_RIGHT_POWER = 40;

const unsigned short MASTER_POWER = 39;
#pragma endregion MISC

#pragma region STATE

#define LED_LEVEL_DEFAULT 24
#define LED_LEVEL_OFF 0
#define LED_LEVEL_LOW 85
#define LED_LEVEL_MID 170
#define LED_LEVEL_MAX 255
unsigned short WhiteLedLevel = LED_LEVEL_DEFAULT;
unsigned short YellowLedLevel = LED_LEVEL_DEFAULT;

void noop(struct StateButton *btn) {}

void debug(struct StateButton *btn) {

  if (btn->counter % 2 == 0)
    Serial.println("RELEASED");
  else
    Serial.println("PUSHED");

  Serial.print("PIN: ");
  Serial.println(btn->PIN);
  Serial.print("STATE: ");
  Serial.println(btn->state);
  Serial.print("COUNTER: ");
  Serial.println(btn->counter);
}

struct StateButton stateButtons[] = {
  { PEDAL, 0U, 0, 0, 0, 0, 0U, handle_pedal },              // 0
  { CRADLE_LEFT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },    // 1
  { CRADLE_LEFT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },   // 2
  { CRADLE_RIGHT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },   // 3
  { CRADLE_RIGHT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },  // 4
  { CRADLE_UP_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },           // 5
  { CRADLE_DOWN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },         // 6
  { GLASS_UP_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },            // 7
  { GLASS_DOWN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },          // 8
  { GLASS_AUTOLEVEL_SENSOR, 0U, 0, 1, 1, 0, 0U, noop },     // 9
};

#pragma endregion STATE

#pragma region METHODS

/// Handle pedal
void handle_pedal(struct StateButton *pedal) {

  if (pedal->counter % 2 == 0) {
    // RELEASED
    digitalWrite(LASER, HIGH);

    analogWrite(LED_WHITE, LED_LEVEL_DEFAULT);
    analogWrite(LED_YELLOW, LED_LEVEL_DEFAULT);
  } else {
    // PUSHED
    digitalWrite(LASER, LOW);

    analogWrite(LED_WHITE, WhiteLedLevel);
    analogWrite(LED_YELLOW, YellowLedLevel);

    Serial.print(CMD_PEDAL_SCAN);
  }
}

/// Toggle scan mode
void set_scan_mode(unsigned short mode) {
}

/// Toggle scan order
void set_scan_order(short order) {
}

/// Start the automatic adjustment process
void automatic_adjustment() {
}

/// Start the calibration process
void calibrate() {
}

/// Start moving both gradles up
void gradle_up() {
}

/// Start moving both gradles down
void gradle_down() {
}

/// Start moving both gradles towards eachother
void gradle_close() {
}

/// Start moving both gradles away from eachother
void gradle_open() {
}

/// Start moving both gradles left
void gradle_left() {
}

/// Start moving both gradles right
void gradle_right() {
}

/// Stop all gradle motors
void gradle_stop() {
}

/// Start moving the left gradle away from the center (left)
/// @sensorsafe
void gradle_left_open() {
}

/// Start moving the right gradle towards the center (right)
/// @sensorsafe
void gradle_left_close() {
}

/// Start moving the right gradle away from the center (right)
/// @sensorsafe
void gradle_right_open() {
}

/// Start moving the right gradle towards the center (left)
/// @sensorsafe
void gradle_right_close() {
}

#pragma endregion METHODS

// handle an incoming message and map it to the correct function
String handle_message(String msg) {

  if (msg == CMD_PING) {
    return CMD_PONG;
  } else if (msg == CMD_LASER_ON) {
    digitalWrite(LASER, HIGH);
    return "laser is on";
  } else if (msg == CMD_LASER_OFF) {
    digitalWrite(LASER, LOW);
    return "laser is off";
  } else if (msg == CMD_LED_MIX_SCORCHING) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 0);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_OFF;
    return "";
  } else if (msg == CMD_LED_MIX_HOT) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 85);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_LOW;
    return "";
  } else if (msg == CMD_LED_MIX_WARM) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 170);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_MID;
    return "";
  } else if (msg == CMD_LED_MIX_AUTO) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  } else if (msg == CMD_LED_MIX_COOL) {
    // analogWrite(LED_YELLOW, 170);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_MID;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  } else if (msg == CMD_LED_MIX_COLD) {
    // analogWrite(LED_YELLOW, 85);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_LOW;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  } else if (msg == CMD_LED_MIX_FREEZING) {
    // analogWrite(LED_YELLOW, 0);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_OFF;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  }

  return "command not found: " + msg;
}

void setup() {

  digitalWrite(LASER, HIGH);
  analogWrite(LED_YELLOW, LED_LEVEL_DEFAULT);
  analogWrite(LED_WHITE, LED_LEVEL_DEFAULT);

  Serial.begin(9600);
  while (!Serial)  // do not continue with the program untill a serial connection to the board has been established
  {};
  Serial.println("TREX READY");
}

// Create a buffer to hold the incoming message
String message = "";
const unsigned int MAX_MESSAGE_LENGTH = 32;

void loop() {

  // Cycle state buttons
  for (int i = 0; i < sizeof(stateButtons) / sizeof(struct StateButton); i++)
    state_button_check(&stateButtons[i]);

  // doChores();

  while (Serial.available() > 0) {
    // received message
    char inByte = Serial.read();

    if (inByte == '\n' || inByte == '\0') {
      // message ended

      String response = handle_message(message);
      Serial.print(response);
      message = "";

    } else {
      // message incoming
      message += inByte;
    }
  }
}