#include "servo.h"
#include "config.h"

void turn_partition(Servo &partition) {
    partition.write(90);
}

void return_partition(Servo &partition) {
    partition.write(0);
}
