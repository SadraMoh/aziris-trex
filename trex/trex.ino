#include "queue.h"
#include "state_button.h"

#define SUCCESS = 0;
#define ERROR = 1;

// #region CRADLE
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
// #endregion CRADLE

// #region GLASS
const unsigned short GLASS_UP_MOTOR = 31;
const unsigned short GLASS_DOWN_MOTOR = 29;
const unsigned short GLASS_UP_SENSOR = 28;
const unsigned short GLASS_DOWN_SENSOR = 30;

const unsigned short GLASS_AUTOLEVEL_SENSOR = 36;
// #endregion GLASS

const unsigned short LED_WHITE = 7;
const unsigned short LED_YELLOW = 6;

const unsigned short LASER = 37;

const unsigned short PEDAL = 26;

const unsigned short CAMERA_LEFT_POWER = 38;
const unsigned short CAMERA_RIGHT_POWER = 40;

const unsigned short MASTER_POWER = 39;


/// toggle scan mode
void set_scan_mode(unsigned short mode) {
}

/// toggle scan order
void set_scan_order(short order) {
}

/// start the automatic adjustment process
void automatic_adjustment() {
}

/// start the calibration process
void calibrate() {
}

/// start moving both gradles up
void gradle_up() {
}

/// start moving both gradles down
void gradle_down() {
}

/// start moving both gradles towards eachother
void gradle_close() {
}

/// start moving both gradles away from eachother
void gradle_open() {
}

/// start moving both gradles left
void gradle_left() {
}

/// start moving both gradles right
void gradle_right() {
}

/// stop all gradle motors
void gradle_stop() {
}

/// start moving the left gradle away from the center (left)
/// @sensorsafe
void gradle_left_open() {
}

/// start moving the right gradle towards the center (right)
/// @sensorsafe
void gradle_left_close() {
}

/// start moving the right gradle away from the center (right)
/// @sensorsafe
void gradle_right_open() {
}

/// start moving the right gradle towards the center (left)
/// @sensorsafe
void gradle_right_close() {
}

// struct StateButton stateButtons[] = {
//     {4U, 0U, 0, 0, 0, 0, 0U, nothing},                     // 0  Buzzer
//     {13U, 0U, 0, 0, 0, 0, 0U, nothing},                    // 1  LED Mode (DEPRECATED)
//     {A0, 0U, 0, 0, 0, 0, 0U, open_cradle},                 // 2  Cradle Open Button
//     {A1, 0U, 0, 0, 0, 0, 0U, close_cradle},                // 3  Cradle Close Button
//     {A2, 0U, 0, 0, 0, 0, 0U, handle_pedal},                // 4  Pedal
//     {A3, 0U, 0, 0, 0, 0, 0U, switch_ord},                  // 5  Scan Order Selector
//     {A4, 0U, 0, 0, 0, 0, 0U, switch_mode},                 // 6  Scan Mode & Calibration
//     {A5, 0U, 0, 0, 0, 0, 0U, handle_scan_button},          // 7  Scan
//     {8U, 0U, 0, 1, 1, 0, 0U, handle_glass_down_sensor},    // 8  Glass Down Sensor
//     {7U, 0U, 0, 1, 1, 0, 0U, handle_glass_up_sensor},      // 9  Glass Up Sensor
//     {12U, 0U, 0, 1, 1, 0, 0U, handle_cradle_open_sensor},  // 10 Cradle Open Sensor  (BLUE)
//     {11U, 0U, 0, 1, 1, 0, 0U, handle_cradle_close_sensor}, // 11 Cradle Clsoe Sensor (GREEN)
// };

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
  }
  else if (msg == CMD_LED_MIX_SCORCHING) {
    analogWrite(LED_YELLOW, 255);
    analogWrite(LED_WHITE, 0);
    return "\0";
  } else if (msg == CMD_LED_MIX_HOT) {
    analogWrite(LED_YELLOW, 255);
    analogWrite(LED_WHITE, 85);
    return "\0";
  } else if (msg == CMD_LED_MIX_WARM) {
    analogWrite(LED_YELLOW, 255);
    analogWrite(LED_WHITE, 170);
    return "\0";
  } else if (msg == CMD_LED_MIX_AUTO) {
    analogWrite(LED_YELLOW, 255);
    analogWrite(LED_WHITE, 255);
    return "\0";
  } else if (msg == CMD_LED_MIX_COOL) {
    analogWrite(LED_YELLOW, 170);
    analogWrite(LED_WHITE, 255);
    return "\0";
  } else if (msg == CMD_LED_MIX_COLD) {
    analogWrite(LED_YELLOW, 85);
    analogWrite(LED_WHITE, 255);
    return "\0";
  } else if (msg == CMD_LED_MIX_FREEZING) {
    analogWrite(LED_YELLOW, 0);
    analogWrite(LED_WHITE, 255);
    return "\0";
  }

  return "command not found: " + msg;
}

void setup() {
  Serial.begin(9600);
  while (!Serial)  // do not continue with the program untill a serial connection to the board has been established
  {};
  Serial.println("TREX READY");
}

const unsigned int MAX_MESSAGE_LENGTH = 32;

// Create a place to hold the incoming message
String message = "";

void loop() {

  // while (Serial.available() > 0) {
  //   // received message
  //   int inByte = Serial.read();
  //   int response = handle_message(inByte);
  //   Serial.print(response);
  // }

  while (Serial.available() > 0) {
    // received message
    char inByte = Serial.read();

    if (inByte == '\n' || inByte == '\0') {
      // message ended

      String response = handle_message(message);
      Serial.print(response + '\0');
      message = ""; 

    } else {
      // message incoming
      message += inByte;
    }
  }

  // // Check to see if anything is available in the serial receive buffer
  // while (Serial.available() > 0) {
  //   // Read the next available byte in the serial receive buffer
  //   char inByte = Serial.read();

  //   // Message coming in (check not terminating character) and guard for over message size
  //   if (inByte == '\0' || (message_pos < MAX_MESSAGE_LENGTH - 1)) {

  //     // Add null character to string
  //     message[message_pos] = '\0';

  //     // Print the message (or do other things)
  //     Serial.println(message);

  //     // Reset for the next message
  //     message_pos = 0;

  //     continue;
  //   }
  //   // Full message received...

  //   // Add the incoming byte to our message
  //   message[message_pos] = inByte;
  //   message_pos++;
  // }
}