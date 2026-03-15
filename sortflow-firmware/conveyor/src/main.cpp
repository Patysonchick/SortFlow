#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>

#include "config.h"
#include "funcs.h"

// Ядро 0 - работа с сетью
// Ядро 1 - обработка датчиков
SemaphoreHandle_t stateSemaphore;

void TaskHardware(void *pvParameters);
// void TaskNetwork(void *pvParameters);

void setup() {
  Serial.begin(115220);
  Serial2.begin(115200, SERIAL_8N1, PIN_UART2_RX, PIN_UART2_TX); // ESP32-CAM UART2
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  // L298N
  pinMode(PIN_MOTOR_ENA, OUTPUT);
  pinMode(PIN_MOTOR_IN1, OUTPUT);
  pinMode(PIN_MOTOR_IN2, OUTPUT);

  // SG90
  pinMode(PIN_SERVO_1, OUTPUT);
  pinMode(PIN_SERVO_2, OUTPUT);
  pinMode(PIN_SERVO_3, OUTPUT);

  // KY-032
  // 34, 35 - ONLY INPUT!!!
  pinMode(PIN_SENSOR_1, INPUT);
  pinMode(PIN_SENSOR_2, INPUT);

  pinMode(LED_BUILTIN, OUTPUT);

  while(!Serial) {}
  Serial.println(F("Conveyor starting..."));

  Serial.println(F("Setuping UART2..."));
  while(!Serial2) {}
  Serial2.println(F("UART2 started!"));

  blink_n(LED_BUILTIN, 2, 100); // UART inited signal
  
  while(WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println(F("Connecting to WiFi..."));
    blink_n(LED_BUILTIN, 2, 50);
  }
  Serial.println(F("Connected to WiFi!"));
  blink_n(LED_BUILTIN, 3, 250); // WiFi inited signal

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

  Serial.println(F("Init complete!"));
  blink_n(LED_BUILTIN, 4, 250);
}

void loop() {
  vTaskDelay(1000 / portTICK_PERIOD_MS); 
}

void TaskHardware(void *pvParameters) {
  for(;;) {
    touch_heartbeat(PIN_TOUCH);

    vTaskDelay(50 / portTICK_PERIOD_MS);
  }
}

// void TaskNetwork(void *pvParameters) {
//   // TODO!
// }