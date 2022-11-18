#include "queue.h"
#include "state_button.h"

#define SUCCESS = 0;
#define ERROR = 1;

const unsigned int MAX_MESSAGE_LENGTH = 32;

// Create a place to hold the incoming message
static char message[MAX_MESSAGE_LENGTH];
static unsigned int message_pos = 0;

/// toggle scan mode
unsigned short set_scan_mode(unsigned short mode) {

}

/// toggle scan order
unsigned short set_scan_order(unsigned short order) {

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

void setup()
{
  Serial.begin(9600);
  while (!Serial) // do not continue with the program untill a serial connection to the board has been established
  {};
  Serial.println("TREX READY");
}

void loop()
{

  // Check to see if anything is available in the serial receive buffer
  while (Serial.available() > 0)
  {
    // Read the next available byte in the serial receive buffer
    char inByte = Serial.read();

    // Message coming in (check not terminating character) and guard for over message size
    if (inByte == '\n' || (message_pos < MAX_MESSAGE_LENGTH - 1))
    {

      // Add null character to string
      message[message_pos] = '\0';

      // Print the message (or do other things)
      Serial.println(message);

      // Reset for the next message
      message_pos = 0;

      continue;
    }
    // Full message received...

      // Add the incoming byte to our message
      message[message_pos] = inByte;
      message_pos++;
  }
}
