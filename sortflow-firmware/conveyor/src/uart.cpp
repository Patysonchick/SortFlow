#include "uart.h"

void requestUuid() {
    Serial2.println("!uuid");
}


String getUuid() {
    requestUuid();

    if(Serial2.available()) {
        String data = Serial2.readStringUntil('\n');
        data.trim();

        if(data.startsWith("@uuid:")) {
            String uuid = data.substring(6); 
            Serial.println("Got UUID: " + uuid); 
            
            return uuid;
        }
    }

    return "";
}