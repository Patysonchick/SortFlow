#include <Arduino.h>

void setup() {
  Serial.begin(115220);

  while(!Serial) {}
}

void loop() {
  for(uint8_t i=0; i<100; ++i) {
    Serial.println(i);

    delay(100);
  }
}