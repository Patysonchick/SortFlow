#pragma once

#include <Arduino.h>

void requestUuid();
String getUuid(); // v7

// UART 2 commands(Q/A - !/@):
// - !uuid:/@uuid: - Get UUID from QR
