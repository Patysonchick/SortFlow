#include "uart.h"

void requestUuid() {
    Serial2.println(F("!uuid"));
}

String getUuid() {
    requestUuid();

    unsigned long startTime = millis();
    while (Serial2.available() == 0 && (millis() - startTime < 1500)) {
        vTaskDelay(10 / portTICK_PERIOD_MS);
    }

    if(Serial2.available()) {
        String data = Serial2.readStringUntil('\n');
        data.trim();

        if(data.startsWith("@uuid:")) {
            String uuid = data.substring(6); 
            Serial.println("Got UUID: " + uuid); 
            
            return uuid;
        }
    }

    Serial.println(F("Cam timeout!!!"));
    return "";
}