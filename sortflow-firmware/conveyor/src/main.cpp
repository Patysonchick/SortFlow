#include <Arduino.h>
#include "funcs.h"
#include <WiFi.h>
#include <HTTPClient.h>

const uint8_t TOUCH_PIN = 4;

// TODO!
const char* WIFI_SSID = "OnePlus Ace 2 Pro";
const char* WIFI_PASSWORD = "123456711";

const char* serverAllocateUrl = "192.168.1.100:3000/api/bin/allocate";

// Ядро 0 - работа с сетью
// Ядро 1 - обработка датчиков
SemaphoreHandle_t stateSemaphore;

void TaskHardware(void *pvParameters);
// void TaskNetwork(void *pvParameters);

void setup() {
  Serial.begin(115220);
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  pinMode(LED_BUILTIN, OUTPUT);

  while(!Serial) {}
  Serial.println(F("Conveyor starting..."));
  blink_n(LED_BUILTIN, 2, 100);
  
  while(WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println(F("Connecting to WiFi..."));
    blink_n(LED_BUILTIN, 2, 50);
  }
  Serial.println(F("Connected to WiFi!"));
  blink_n(LED_BUILTIN, 3, 250);

  stateSemaphore = xSemaphoreCreateBinary();

  xTaskCreatePinnedToCore(
    TaskHardware,
    "HardwareTask",
    2048,
    NULL,
    1,
    NULL,
    1
  );

  // xTaskCreatePinnedToCore(
  //   TaskNetwork,
  //   "NetworkTask",
  //   8192,
  //   NULL,
  //   1,
  //   NULL,
  //   0
  // );
}

void loop() {
  vTaskDelay(1000 / portTICK_PERIOD_MS); 
}

void TaskHardware(void *pvParameters) {
  for(;;) {
    touch_heartbeat(TOUCH_PIN);

    vTaskDelay(50 / portTICK_PERIOD_MS);
  }
}

// void TaskNetwork(void *pvParameters) {
//   // TODO!
// }