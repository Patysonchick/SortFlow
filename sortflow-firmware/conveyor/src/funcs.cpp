#include "funcs.h"

void blink_n(uint8_t pin, uint16_t n, uint32_t d) {
  for(uint16_t i=0; i<n; i++) {
    digitalWrite(pin, HIGH);
    delay(d);
    digitalWrite(pin, LOW);
    delay(d);
  }
}

void touch_heartbeat(uint8_t pin) {
  int val = touchRead(pin);
  if(val<50) {
    digitalWrite(LED_BUILTIN, HIGH);
    Serial.println("Touched!");
  } else {
    digitalWrite(LED_BUILTIN, LOW);
    // Serial.println(val);
  }
}
