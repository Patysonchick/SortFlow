#include <Arduino.h>
#include <ESP32QRCodeReader.h>

ESP32QRCodeReader reader(CAMERA_MODEL_AI_THINKER);
struct QRCodeData qrCodeData;

void setup() {
  Serial.begin(115200);

  ledcSetup(0, 5000, 8);
  ledcAttachPin(4, 0);

  ledcWrite(0, 20); 
  Serial.println(F("ESP32-CAM initing..."));
  
  reader.setup();
  reader.beginOnCore(1);

  Serial.println(F("ESP32-CAM started!"));
  ledcWrite(0, 0);
}

void loop() {
  if(reader.receiveQrCode(&qrCodeData, 100)) {
    Serial.println(F("Scanned"));
    if(qrCodeData.valid) {
      ledcWrite(0, 20);

      Serial.print(F("Found code: "));
      Serial.println((const char *)qrCodeData.payload);
      
      Serial.print(F("@uuid:"));
      Serial.println((const char *)qrCodeData.payload);

      delay(100);
      ledcWrite(0, 0);
    } else Serial.println(F("Not valid!"));
  } 

  delay(1);
}
