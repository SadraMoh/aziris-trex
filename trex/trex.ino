#include "queue.h"
#include "state_button.h"

#pragma region DEFINITIONS

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

#define CMD_MODE_AUTO "mode_auto"
#define CMD_MODE_PANEL "mode_panel"
#define CMD_MODE_PEDAL "mode_pedal"

#define CMD_LED_MIX_SCORCHING "led_scorch"
#define CMD_LED_MIX_HOT "led_hot"
#define CMD_LED_MIX_WARM "led_warm"
#define CMD_LED_MIX_AUTO "led_auto"
#define CMD_LED_MIX_COOL "led_cool"
#define CMD_LED_MIX_COLD "led_cold"
#define CMD_LED_MIX_FREEZING "led_freeze"

#define CMD_PEDAL_SCAN "pedal_scan"

#define CMD_CRADLE_UP "cradle_up"
#define CMD_CRADLE_UP_STOP "cradle_up_stop"
#define CMD_CRADLE_DOWN "cradle_down"
#define CMD_CRADLE_DOWN_STOP "cradle_down_stop"

#define CMD_CRADLE_CLOSE "cradle_close"
#define CMD_CRADLE_CLOSE_STOP "cradle_close_stop"
#define CMD_CRADLE_OPEN "cradle_open"
#define CMD_CRADLE_OPEN_STOP "cradle_open_stop"
#define CMD_CRADLE_LEFT "cradle_left"
#define CMD_CRADLE_LEFT_STOP "cradle_left_stop"
#define CMD_CRADLE_RIGHT "cradle_right"
#define CMD_CRADLE_RIGHT_STOP "cradle_right_stop"

#define CMD_CRADLE_STALLED "cradle_stalled"

#define CMD_CALIBRATION_START "calibration_start"
#define CMD_CALIBRATION_STOP "calibration_stop"
#define CMD_CALIBRATION_IMMINENT "calibration_imminent"
#define CMD_CALIBRATION_TIMEOUT "calibration_timeout"
#define CMD_CALIBRATION_PREVENTED "calibration_prevented"

#define CMD_ADJUST_START "adjust_start"
#define CMD_ADJUST_STOP "adjust_stop"

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

#pragma endregion DEFINITIONS

#pragma region STATE

#define MOTOR_SPEED 255
#define MOTOR_SAFETY_DELAY 200

#define LED_LEVEL_OFF 0
#define LED_LEVEL_LOW 85
#define LED_LEVEL_MID 170
#define LED_LEVEL_MAX 255
#define LED_LEVEL_DEFAULT 24
unsigned short WhiteLedLevel = LED_LEVEL_MAX;
unsigned short YellowLedLevel = LED_LEVEL_MAX;

unsigned long CALIBRATION_DURATION = 1000U * 60U * 1U;
bool IsCalibrating = false;

bool IsAdjusting = false;

#define MODE_AUTO 0
#define MODE_PANEL 1
#define MODE_PEDAL 2
unsigned short ScanMode = MODE_AUTO;

#pragma endregion STATE

#pragma region METHODS

void handle_glass_up_sensor(struct StateButton *sensor) {

  if (sensor->counter % 2 == 0) {
    // RELEASED

  } else {
    // PUSHED

    if (ScanMode == MODE_PANEL || ScanMode == MODE_AUTO) {
    }

    digitalWrite(GLASS_UP_MOTOR, HIGH);  // stop
  }
}

void handle_glass_down_sensor(struct StateButton *sensor) {

  if (sensor->counter % 2 == 0) {
    // RELEASED

    if (ScanMode == MODE_PANEL || ScanMode == MODE_AUTO) {
      digitalWrite(LASER, HIGH);

      analogWrite(LED_WHITE, LED_LEVEL_DEFAULT);
      analogWrite(LED_YELLOW, LED_LEVEL_DEFAULT);
    }

  } else {
    // PUSHED

    if (ScanMode == MODE_PANEL || ScanMode == MODE_AUTO) {
      digitalWrite(LASER, LOW);

      analogWrite(LED_WHITE, WhiteLedLevel);
      analogWrite(LED_YELLOW, YellowLedLevel);
    }

    digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop

    if (ScanMode == MODE_AUTO && IsCalibrating == false && IsAdjusting == false) {
      Serial.println(CMD_PEDAL_SCAN);
      delay(1700);
    }
  }
}

void handle_left_open_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 0)
    return;

  analogWrite(CRADLE_LEFT_OPEN_MOTOR, LOW);    // stop
  analogWrite(CRADLE_RIGHT_CLOSE_MOTOR, LOW);  // stop
  Serial.print(CMD_CRADLE_STALLED);
}

void handle_left_close_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 0)
    return;

  analogWrite(CRADLE_LEFT_CLOSE_MOTOR, LOW);  // stop
  analogWrite(CRADLE_RIGHT_OPEN_MOTOR, LOW);  // stop
  Serial.print(CMD_CRADLE_STALLED);
}

void handle_right_open_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 0)
    return;

  analogWrite(CRADLE_RIGHT_OPEN_MOTOR, LOW);  // stop
  analogWrite(CRADLE_LEFT_CLOSE_MOTOR, LOW);  // stop
  Serial.print(CMD_CRADLE_STALLED);
}

void handle_right_close_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 0)
    return;

  analogWrite(CRADLE_RIGHT_CLOSE_MOTOR, LOW);  // stop
  analogWrite(CRADLE_LEFT_OPEN_MOTOR, LOW);    // stop
  Serial.print(CMD_CRADLE_STALLED);
}

void handle_cradle_up_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 1)
    return;

  digitalWrite(CRADLE_UP_MOTOR, LOW);  // stop
  Serial.print(CMD_CRADLE_STALLED);
}

void handle_cradle_down_sensor(struct StateButton *sensor) {
  if (sensor->counter % 2 == 1)
    return;

  digitalWrite(CRADLE_DOWN_MOTOR, LOW);  // stop
  Serial.print(CMD_CRADLE_STALLED);
}

/// Handle pedal
void handle_pedal(struct StateButton *pedal) {

  switch (ScanMode) {
    case MODE_PEDAL:
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
      break;

    case MODE_PANEL:

      if (pedal->counter % 2 == 0) {
        // RELEASED
        spin_glass_up();
      } else {
        // PUSHED
        spin_glass_down();
      }

      break;

    case MODE_AUTO:

      if (pedal->counter % 2 == 0) {
        // RELEASED
        if (digitalRead(GLASS_DOWN_SENSOR) == HIGH) {
          // passed sensor
          spin_glass_up();
        } else {
          // not yet reached the sensor
          spin_glass_up();
        }

      } else {
        // PUSHED
        spin_glass_down();
      }

      break;
  }
}

// Attempts to spin the glass up
// Does nothing if the up sensor won't allow it
bool spin_glass_up() {
  if (digitalRead(GLASS_UP_SENSOR) == LOW)
    return false;

  if (digitalRead(GLASS_DOWN_MOTOR) == LOW) {  // if moving
    digitalWrite(GLASS_DOWN_MOTOR, HIGH);      // stop
    delay(MOTOR_SAFETY_DELAY);                 // wait
  }

  digitalWrite(GLASS_UP_MOTOR, LOW);  // start
  delay(MOTOR_SAFETY_DELAY);
  digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop
  return true;
}

// Attempts to spin the glass down
// Does nothing if the down sensor won't allow it
bool spin_glass_down() {
  if (digitalRead(GLASS_DOWN_SENSOR) == LOW)
    return false;

  if (digitalRead(GLASS_UP_MOTOR) == LOW) {  // if moving
    digitalWrite(GLASS_UP_MOTOR, HIGH);      // stop
    delay(MOTOR_SAFETY_DELAY);               // wait
  }

  digitalWrite(GLASS_DOWN_MOTOR, LOW);  // start
  digitalWrite(GLASS_UP_MOTOR, HIGH);   // stop
  delay(MOTOR_SAFETY_DELAY);
  return true;
}

bool spin_cradle_down() {
  if (digitalRead(CRADLE_DOWN_SENSOR) == LOW)
    return false;

  digitalWrite(CRADLE_UP_MOTOR, LOW);     // stop
  digitalWrite(CRADLE_DOWN_MOTOR, HIGH);  // start
  return true;
}

bool spin_cradle_up() {
  if (digitalRead(CRADLE_UP_SENSOR) == LOW)
    return false;

  digitalWrite(CRADLE_DOWN_MOTOR, LOW);  // stop
  digitalWrite(CRADLE_UP_MOTOR, HIGH);   // start
  return true;
}

bool spin_cradle_left_open() {
  if (digitalRead(CRADLE_LEFT_OPEN_SENSOR) == HIGH)
    return false;

  analogWrite(CRADLE_LEFT_OPEN_MOTOR, MOTOR_SPEED - 20);
}

bool spin_cradle_left_close() {
  if (digitalRead(CRADLE_LEFT_CLOSE_SENSOR) == HIGH)
    return false;

  analogWrite(CRADLE_LEFT_CLOSE_MOTOR, MOTOR_SPEED - 20);
}

bool spin_cradle_right_open() {
  if (digitalRead(CRADLE_RIGHT_OPEN_SENSOR) == HIGH)
    return false;

  analogWrite(CRADLE_RIGHT_OPEN_MOTOR, MOTOR_SPEED);
}

bool spin_cradle_right_close() {
  if (digitalRead(CRADLE_RIGHT_CLOSE_SENSOR) == HIGH)
    return false;

  analogWrite(CRADLE_RIGHT_CLOSE_MOTOR, MOTOR_SPEED);
}

/// Turn laser On
String laser_on(String msg) {
  digitalWrite(LASER, HIGH);
  return msg;
}

/// Turn laser Off
String laser_off(String msg) {
  digitalWrite(LASER, LOW);
  return msg;
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

/// Start the automatic adjustment process
String adjust_start(String msg) {

  Serial.println(CMD_ADJUST_START);

  while (spin_cradle_down() == true) {}
  digitalWrite(CRADLE_DOWN_MOTOR, LOW);  // stop

  while (spin_glass_down() == true) {}
  digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop

  spin_cradle_up();
  while (digitalRead(CRADLE_UP_SENSOR) == HIGH && digitalRead(GLASS_AUTOLEVEL_SENSOR) == HIGH) {};
  analogWrite(CRADLE_UP_MOTOR, LOW);

  while (spin_glass_up() == true)
    ;
  digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop

  return CMD_ADJUST_STOP;
}

/// Switch between modes
String change_mode(String msg) {

  if (msg == CMD_MODE_PEDAL) {
    ScanMode = MODE_PEDAL;
    spin_glass_down();
  }

  if (msg == CMD_MODE_PANEL) {
    ScanMode = MODE_PANEL;
    spin_glass_up();
  }

  if (msg == CMD_MODE_AUTO) {
    ScanMode = MODE_AUTO;
    spin_glass_up();
  }

  return msg;
}

/// Start the calibration process
String calibration_start(String msg) {

  if (IsCalibrating == true)
    return CMD_CALIBRATION_PREVENTED;

  // start
  IsCalibrating = true;
  Serial.print(CMD_CALIBRATION_START);

  while (spin_glass_down() == true) {

    String cmd = check_for_cmd();

    if (cmd == CMD_CALIBRATION_STOP || cmd == CMD_CALIBRATION_START) {
      digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop
      return calibration_stop(cmd);
    }
  };
  digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop


  digitalWrite(LASER, LOW);

  analogWrite(LED_WHITE, WhiteLedLevel);
  analogWrite(LED_YELLOW, YellowLedLevel);

  // wait
  unsigned long then = millis();
  while (millis() < then + CALIBRATION_DURATION - 1000U * 30U) {
    String cmd = check_for_cmd();

    if (cmd == CMD_CALIBRATION_STOP || cmd == CMD_CALIBRATION_START)
      return calibration_stop(cmd);
  }
  Serial.print(CMD_CALIBRATION_IMMINENT);
  while (millis() < then + CALIBRATION_DURATION) {
    // wait until calibration duration ends or user cancels calibration
    String cmd = check_for_cmd();

    if (cmd == CMD_CALIBRATION_STOP || cmd == CMD_CALIBRATION_START)
      return calibration_stop(cmd);
  }

  return calibration_stop(CMD_CALIBRATION_TIMEOUT);
}

String calibration_stop(String msg) {
  // start

  IsCalibrating = false;
  if (ScanMode == MODE_AUTO || ScanMode == MODE_PANEL)
    spin_glass_up();

  return msg;
}

/// Start moving both cradles up
String cradle_up(String msg) {
  if (spin_cradle_up())
    return msg;

  return CMD_CRADLE_STALLED;
}

/// Start moving both cradles down
String cradle_down(String msg) {
  if (spin_cradle_down())
    return msg;

  return CMD_CRADLE_STALLED;
}

/// Start moving both cradles towards eachother
String cradle_close(String msg) {
  spin_cradle_left_close();
  spin_cradle_right_close();

  return msg;
}

/// Start moving both cradles away from eachother
String cradle_open(String msg) {
  spin_cradle_left_open();
  spin_cradle_right_open();

  return msg;
}

/// Start moving both cradles left
String cradle_left(String msg) {
  if (digitalRead(CRADLE_RIGHT_CLOSE_SENSOR) == HIGH)
    return CMD_CRADLE_STALLED;

  if (digitalRead(CRADLE_LEFT_OPEN_SENSOR) == HIGH)
    return CMD_CRADLE_STALLED;

  spin_cradle_left_open();
  spin_cradle_right_close();

  return msg;
}

/// Start moving both cradles right
String cradle_right(String msg) {
  if (digitalRead(CRADLE_LEFT_CLOSE_SENSOR) == HIGH)
    return CMD_CRADLE_STALLED;

  if (digitalRead(CRADLE_RIGHT_OPEN_SENSOR) == HIGH)
    return CMD_CRADLE_STALLED;

  spin_cradle_left_close();
  spin_cradle_right_open();

  return msg;
}

/// Stop all cradle motors
String cradle_stop(String msg) {
  analogWrite(CRADLE_UP_MOTOR, LOW);           // stop
  analogWrite(CRADLE_DOWN_MOTOR, LOW);         // stop
  analogWrite(CRADLE_LEFT_OPEN_MOTOR, LOW);    // stop
  analogWrite(CRADLE_LEFT_CLOSE_MOTOR, LOW);   // stop
  analogWrite(CRADLE_RIGHT_OPEN_MOTOR, LOW);   // stop
  analogWrite(CRADLE_RIGHT_CLOSE_MOTOR, LOW);  // stop
  return msg;
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

#pragma region CONFIG

struct StateButton stateButtons[] = {
  // PIN | DEB | reading | last_reading | state | disabled | counter
  { PEDAL, 0U, 0, 0, 0, 0, 0U, handle_pedal },                                   // 0
  { CRADLE_LEFT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_left_open_sensor },      // 1
  { CRADLE_LEFT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_left_close_sensor },    // 2
  { CRADLE_RIGHT_OPEN_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_right_open_sensor },    // 3
  { CRADLE_RIGHT_CLOSE_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_right_close_sensor },  // 4
  { CRADLE_UP_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_cradle_up_sensor },             // 5
  { CRADLE_DOWN_SENSOR, 0U, 0, 0, 0, 0, 0U, handle_cradle_down_sensor },         // 6
  { GLASS_UP_SENSOR, 0U, 0, 1, 1, 0, 0U, handle_glass_up_sensor },               // 7
  { GLASS_DOWN_SENSOR, 0U, 0, 1, 1, 0, 0U, handle_glass_down_sensor },           // 8
  { GLASS_AUTOLEVEL_SENSOR, 0U, 0, 1, 1, 0, 0U, noop },                          // 9
};

// Map commands to functions
struct CommandMap commandMappings[] {
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
    { CMD_CRADLE_UP, cradle_up },
    { CMD_CRADLE_UP_STOP, cradle_stop },
    { CMD_CRADLE_DOWN, cradle_down },
    { CMD_CRADLE_DOWN_STOP, cradle_stop },
    { CMD_CRADLE_CLOSE, cradle_close },
    { CMD_CRADLE_CLOSE_STOP, cradle_stop },
    { CMD_CRADLE_OPEN, cradle_open },
    { CMD_CRADLE_OPEN_STOP, cradle_stop },
    { CMD_CRADLE_LEFT, cradle_left },
    { CMD_CRADLE_LEFT_STOP, cradle_stop },
    { CMD_CRADLE_RIGHT, cradle_right },
    { CMD_CRADLE_RIGHT_STOP, cradle_stop },
    { CMD_CALIBRATION_START, calibration_start },
    { CMD_CALIBRATION_STOP, calibration_stop },
    { CMD_MODE_PEDAL, change_mode },
    { CMD_MODE_PANEL, change_mode },
    { CMD_MODE_AUTO, change_mode },
    { CMD_ADJUST_START, adjust_start },
};

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

#pragma endregion CONFIG

// handle an incoming message and map it to the correct function
String handle_message(String msg) {

  if (msg == CMD_PING)
    return CMD_PONG;

  for (struct CommandMap map : commandMappings)
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

  digitalWrite(GLASS_DOWN_MOTOR, HIGH);  // stop
  digitalWrite(GLASS_UP_MOTOR, HIGH);    // stop

  spin_glass_up();

  Serial.begin(9600);
  while (!Serial)  // do not continue with the program untill a serial connection to the board has been established
  {};
  Serial.println("TREX READY");

  // Button PinModes and initial state
  for (struct StateButton btn : stateButtons) {
    int reading = digitalRead(btn.PIN);
    btn.state = reading;
    btn.reading = reading;
    btn.last_reading = reading;

    Serial.print(btn.PIN);
    Serial.print(" - ");
    Serial.println(reading);
  }
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