#include <Arduino.h>
#include <WiFi.h>
#include <HTTPClient.h>
#include <ESP32Servo.h>

#include "config.h"
#include "funcs.h"
#include "conveyor.h"
#include "uart.h"
#include "servo.h"

// Ядро 0 - работа с сетью
// Ядро 1 - обработка датчиков

SemaphoreHandle_t canMove;
SemaphoreHandle_t isAllocated;

void TaskHeartbeat(void *pvParameters);
void TaskHardware(void *pvParameters);
void TaskAllocate(void *pvParameters);

Servo partition;

void setup() {
  Serial.begin(115200);
  Serial2.begin(115200); // ESP32-CAM UART2
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  // L298N
  pinMode(PIN_MOTOR_ENA, OUTPUT);
  pinMode(PIN_MOTOR_IN1, OUTPUT);
  pinMode(PIN_MOTOR_IN2, OUTPUT);

  conveyor_forward();
  conveyor_stop();

  // SG90
  // pinMode(PIN_SERVO_1, OUTPUT);
  // pinMode(PIN_SERVO_2, OUTPUT);
  // pinMode(PIN_SERVO_3, OUTPUT);

  partition.attach(PIN_SERVO_4); 
  return_partition(partition);

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

  canMove = xSemaphoreCreateBinary();
  isAllocated = xSemaphoreCreateBinary();

  xTaskCreatePinnedToCore(
    TaskHeartbeat,
    "HeatbeatTask",
    1024,
    NULL,
    2,
    NULL,
    1
  );

  xTaskCreatePinnedToCore(
    TaskHardware,
    "HardwareTask",
    2048,
    NULL,
    1,
    NULL,
    1
  );

  xTaskCreatePinnedToCore(
    TaskAllocate,
    "AllocateTask",
    8192,
    NULL,
    1,
    NULL,
    0
  );

  Serial.println(F("Init complete!"));
  blink_n(LED_BUILTIN, 4, 250);
}

void loop() {
  vTaskDelay(1000 / portTICK_PERIOD_MS); 
}

void TaskHeartbeat(void *pvParameters) {
  for(;;) {
    touch_heartbeat(PIN_TOUCH);

    vTaskDelay(50 / portTICK_PERIOD_MS);
  }
}

void TaskHardware(void *pvParameters) {
  conveyor_forward();

  uint8_t detectCount=0;
  for(;;) {
    if(!digitalRead(PIN_SENSOR_1)) {
      detectCount++;
    } else {
      detectCount=0;
    }

    if(detectCount >= IR_THRESHOLD) {
      conveyor_stop();
      turn_partition(partition); // TODO!
      vTaskDelay(500 / portTICK_PERIOD_MS);
      Serial.println(F("Good detected, stopping conveyor"));

      xSemaphoreGive(isAllocated);
      xSemaphoreTake(canMove, portMAX_DELAY);

      Serial.println(F("Starting conveyor"));
      // partition.write(90);
      return_partition(partition); // TODO!
      vTaskDelay(500 / portTICK_PERIOD_MS);
      conveyor_start();

      vTaskDelay(1000 / portTICK_PERIOD_MS); 

      detectCount = 0;
    }

    vTaskDelay(10 / portTICK_PERIOD_MS);
  }
}

void TaskAllocate(void *pvParameters) {
  // TODO!

  for(;;) {
    if (xSemaphoreTake(isAllocated, portMAX_DELAY) == pdTRUE) {
      
      Serial.println(F("Sending QR-code request"));

      String uuid = getUuid();

      if (uuid.length() > 0) {
        // TODO! POST request for allocate
        Serial.println(F("Pseudo request to API"));
      } else {
        Serial.println(F("QR-code"));
      }

      xSemaphoreGive(canMove);
    }
  }

  // TODO!
}