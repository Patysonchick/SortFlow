#include "conveyor.h"
#include "config.h"

void conveyor_forward() {
    digitalWrite(PIN_MOTOR_ENA, LOW);
    digitalWrite(PIN_MOTOR_IN1, HIGH);
    digitalWrite(PIN_MOTOR_IN2, LOW);
    digitalWrite(PIN_MOTOR_ENA, HIGH);
}

void conveyor_backforward() {
    digitalWrite(PIN_MOTOR_ENA, LOW);
    digitalWrite(PIN_MOTOR_IN1, LOW);
    digitalWrite(PIN_MOTOR_IN2, HIGH);
    digitalWrite(PIN_MOTOR_ENA, HIGH);
}

void conveyor_stop() {
    digitalWrite(PIN_MOTOR_ENA, LOW);
}

void conveyor_start() {
    digitalWrite(PIN_MOTOR_ENA, HIGH);
}
