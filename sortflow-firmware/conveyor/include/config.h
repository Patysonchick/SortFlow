#pragma once

#include <Arduino.h>

// L298N
constexpr uint8_t PIN_MOTOR_ENA = 25;
constexpr uint8_t PIN_MOTOR_IN1 = 26;
constexpr uint8_t PIN_MOTOR_IN2 = 32;

// SG90
constexpr uint8_t PIN_SERVO_1 = 13;
constexpr uint8_t PIN_SERVO_2 = 14;
constexpr uint8_t PIN_SERVO_3 = 27;

// KY-032
// 34, 35 - ONLY INPUT!!!
constexpr uint8_t PIN_SENSOR_1 = 34;
constexpr uint8_t PIN_SENSOR_2 = 35;

// ESP32-CAM UART2
constexpr uint8_t PIN_UART2_TX = 17;
constexpr uint8_t PIN_UART2_RX = 16;

// WiFi
const char* WIFI_SSID = "OnePlus Ace 2 Pro";
const char* WIFI_PASSWORD = "123456711";

// Backend
const char* URL_SERVER_ALLOCATE = "192.168.1.100:3000/api/bin/allocate";
// TODO!

// Miscellaneous
constexpr uint8_t PIN_TOUCH = 4;
