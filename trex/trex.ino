#include "queue.h"
#include "state_button.h"

#pragma region COMMANDS

typedef String (*CommandHandler)(String msg);
struct CommandMap {
  String cmd;
  CommandHandler func;
};

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

#define CRADLE_UP "cradle_up"
#define CRADLE_UP_STOP "cradle_up_stop"
#define CRADLE_DOWN "cradle_down"
#define CRADLE_DOWN_STOP "cradle_down_stop"

#define CALIBRATION_START "calibration_start"
#define CALIBRATION_STOP "calibration_stop"
#define CALIBRATION_TIMEOUT "calibration_timeout"
#define CALIBRATION_PREVENTED "calibration_prevented"

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

unsigned long CALIBRATION_DURATION = 1000U * 60U * 3U;

void noop(struct StateButton *a) {}
String noop() {
  return "";
}

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
  { PEDAL, 0U, 0, 0, 0, 0, 0U, handle_pedal },                            // 0
  { CRADLE_LEFT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                  // 1
  { CRADLE_LEFT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                 // 2
  { CRADLE_RIGHT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                 // 3
  { CRADLE_RIGHT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                // 4
  { CRADLE_UP_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_cradle_up_sensor },      // 5
  { CRADLE_DOWN_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_cradle_down_sensor },  // 6
  { GLASS_UP_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                          // 7
  { GLASS_DOWN_SENSOR, 0U, 0, 0, 0, 0, 0U, noop },                        // 8
  { GLASS_AUTOLEVEL_SENSOR, 0U, 0, 1, 1, 0, 0U, noop },                   // 9
};

#pragma endregion STATE

#pragma region METHODS

void handle_cradle_up_sensor(struct StateButton *btn) {
  if (btn->counter % 2 == 1)
    return;

  digitalWrite(CRADLE_UP_MOTOR, LOW);  // stop
  Serial.print("Up sensor stalled");
}

void handle_cradle_down_sensor(struct StateButton *btn) {
  if (btn->counter % 2 == 1)
    return;

  digitalWrite(CRADLE_DOWN_MOTOR, LOW);  // stop
  Serial.print("Down sensor stalled");
}

// Attempts to spin the glass up
// Does nothing if the up sensor won't allow it
bool spin_glass_up() {
  if (digitalRead(GLASS_UP_SENSOR) == HIGH) {
    digitalWrite(GLASS_UP_MOTOR, LOW);     // start
    digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop
    return true;
  }

  return false;
}

// Attempts to spin the glass down
// Does nothing if the down sensor won't allow it
bool spin_glass_down() {
  if (digitalRead(GLASS_DOWN_SENSOR) == HIGH) {
    digitalWrite(GLASS_DOWN_MOTOR, LOW);  // start
    digitalWrite(GLASS_UP_MOTOR, HIGH);   // stop
    return true;
  }

  return false;
}

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

/// Turn laser On
String laser_on(String msg) {
  digitalWrite(LASER, HIGH);
  return "Laser is on";
}

/// Turn laser Off
String laser_off(String msg) {
  digitalWrite(LASER, LOW);
  return "Laser is off";
}

/// Mix between led
String mix_leds(String msg) {
  if (msg == CMD_LED_MIX_SCORCHING) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 0);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_OFF;
    return "";
  }

  if (msg == CMD_LED_MIX_HOT) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 85);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_LOW;
    return "";
  }

  if (msg == CMD_LED_MIX_WARM) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 170);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_MID;
    return "";
  }

  if (msg == CMD_LED_MIX_AUTO) {
    // analogWrite(LED_YELLOW, 255);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_MAX;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  }

  if (msg == CMD_LED_MIX_COOL) {
    // analogWrite(LED_YELLOW, 170);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_MID;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  }

  if (msg == CMD_LED_MIX_COLD) {
    // analogWrite(LED_YELLOW, 85);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_LOW;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  }

  if (msg == CMD_LED_MIX_FREEZING) {
    // analogWrite(LED_YELLOW, 0);
    // analogWrite(LED_WHITE, 255);
    WhiteLedLevel = LED_LEVEL_OFF;
    YellowLedLevel = LED_LEVEL_MAX;
    return "";
  }

  return "err in mix_leds";
}

/// Toggle scan mode
String set_scan_mode(String msg) {
}

/// Toggle scan order
String set_scan_order(String msg) {
}

/// Start the automatic adjustment process
String automatic_adjustment(String msg) {
}

/// Start the calibration process
String calibration_start(String msg) {

  if (spin_glass_down() == false)
    return CALIBRATION_PREVENTED;


  Serial.print(CALIBRATION_START);

  unsigned long then = millis();

  while (millis() < then + CALIBRATION_DURATION) {
    // wait till calibration duration ends or user cancels calibration

    String cmd = check_for_cmd();

    if (cmd == CALIBRATION_STOP)
      return calibration_stop(cmd);
  }

  return CALIBRATION_TIMEOUT;
}

String calibration_stop(String msg) {



  return CALIBRATION_STOP;
}

/// Start moving both cradles up
String cradle_up(String msg) {
  if (digitalRead(CRADLE_UP_SENSOR) == LOW)
    return "Up sensor stalled";

  digitalWrite(CRADLE_DOWN_MOTOR, LOW);  // stop
  digitalWrite(CRADLE_UP_MOTOR, HIGH);   // start
  return "Cradle moving up";
}

/// Start moving both cradles down
String cradle_down(String msg) {
  if (digitalRead(CRADLE_DOWN_SENSOR) == LOW)
    return "Down sensor stalled";

  digitalWrite(CRADLE_UP_MOTOR, LOW);     // stop
  digitalWrite(CRADLE_DOWN_MOTOR, HIGH);  // start
  return "Cradle moving down";
}

/// Start moving both cradles towards eachother
String cradle_close(String msg) {
}

/// Start moving both cradles away from eachother
String cradle_open(String msg) {
}

/// Start moving both cradles left
String cradle_left(String msg) {
}

/// Start moving both cradles right
String cradle_right(String msg) {
}

/// Stop all cradle motors
String cradle_stop(String msg) {
  analogWrite(CRADLE_UP_MOTOR, LOW);           // stop
  analogWrite(CRADLE_DOWN_MOTOR, LOW);         // stop
  analogWrite(CRADLE_LEFT_OPEN_MOTOR, LOW);    // stop
  analogWrite(CRADLE_LEFT_CLOSE_MOTOR, LOW);   // stop
  analogWrite(CRADLE_RIGHT_OPEN_MOTOR, LOW);   // stop
  analogWrite(CRADLE_RIGHT_CLOSE_MOTOR, LOW);  // stop
  return "Cradle stopped";
}

/// Start moving the left cradle away from the center (left)
/// @sensorsafe
String cradle_left_open(String msg) {
}

/// Start moving the right cradle towards the center (right)
/// @sensorsafe
String cradle_left_close(String msg) {
}

/// Start moving the right cradle away from the center (right)
/// @sensorsafe
String cradle_right_open(String msg) {
}

/// Start moving the right cradle towards the center (left)
/// @sensorsafe
String cradle_right_close(String msg) {
}

#pragma endregion METHODS

// Map commands to functions
struct CommandMap commandMap[] {
  // {
  //   CMD_PING,
  //   [](String msg) {
  //     return (String)CMD_PONG;
  //   }
  // },
  { CMD_LASER_ON, laser_on },
    { CMD_LASER_OFF, laser_off },
    { CMD_LED_MIX_SCORCHING, mix_leds },
    { CMD_LED_MIX_HOT, mix_leds },
    { CMD_LED_MIX_WARM, mix_leds },
    { CMD_LED_MIX_AUTO, mix_leds },
    { CMD_LED_MIX_COOL, mix_leds },
    { CMD_LED_MIX_COLD, mix_leds },
    { CMD_LED_MIX_FREEZING, mix_leds },
    { CRADLE_UP, cradle_up },
    { CRADLE_UP_STOP, cradle_stop },
    { CRADLE_DOWN, cradle_down },
    { CRADLE_DOWN_STOP, cradle_stop },
    { CALIBRATION_START, calibration_start },
    { CALIBRATION_STOP, calibration_stop },
};

// handle an incoming message and map it to the correct function
String handle_message(String msg) {

  if (msg == CMD_PING)
    return CMD_PONG;

  for (struct CommandMap map : commandMap)
    if (msg == map.cmd)
      return map.func(msg);

  return "ENOCMD: " + msg;
}

void setup() {

  // Button PinModes
  for (struct StateButton btn : stateButtons)
    pinMode(btn.PIN, INPUT);

  pinMode(LASER, OUTPUT);
  pinMode(LED_YELLOW, OUTPUT);
  pinMode(LED_WHITE, OUTPUT);

  pinMode(CRADLE_RIGHT_CLOSE_MOTOR, OUTPUT);
  pinMode(CRADLE_RIGHT_OPEN_MOTOR, OUTPUT);
  pinMode(CRADLE_LEFT_CLOSE_MOTOR, OUTPUT);
  pinMode(CRADLE_LEFT_OPEN_MOTOR, OUTPUT);
  pinMode(CRADLE_UP_MOTOR, OUTPUT);
  pinMode(CRADLE_DOWN_MOTOR, OUTPUT);
  pinMode(GLASS_UP_MOTOR, OUTPUT);
  pinMode(GLASS_DOWN_MOTOR, OUTPUT);

  digitalWrite(LASER, HIGH);
  analogWrite(LED_YELLOW, LED_LEVEL_DEFAULT);
  analogWrite(LED_WHITE, LED_LEVEL_DEFAULT);

  Serial.begin(9600);
  while (!Serial)  // do not continue with the program untill a serial connection to the board has been established
  {};
  Serial.println("TREX READY");
}


void loop() {

  // Cycle state buttons
  for (int i = 0; i < sizeof(stateButtons) / sizeof(struct StateButton); i++)
    state_button_check(&stateButtons[i]);

  // doChores();

  String message = check_for_cmd();

  if (message == "") return;
  
  String response = handle_message(message);
  Serial.print(response);
}

// Create a buffer to hold the incoming message
String message = "";
const unsigned int MAX_MESSAGE_LENGTH = 32;
String check_for_cmd() {
  while (Serial.available() > 0) {
    // received message
    char inByte = Serial.read();

    if (inByte == '\n' || inByte == '\0') {
      // message ended

      String temp = message;
      message = "";

      return temp;

    } else {
      // message incoming
      message += inByte;
    }
  }

  return "";
}