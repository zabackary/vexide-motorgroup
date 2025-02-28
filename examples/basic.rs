#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use alloc::vec;
use vexide_motorgroup::MotorGroup;

use vexide::prelude::*;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut motor_group = MotorGroup::new(vec![
        Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ]);

    // Set the motor group's target to a voltage
    motor_group.set_voltage(5.0).unwrap();
    sleep(Duration::from_secs(1)).await;

    // Set the motor group's target to a position
    motor_group
        .set_position_target(Position::from_degrees(90.0), 200)
        .unwrap();
    sleep(Duration::from_secs(1)).await;

    // Set the motor group's target to a velocity
    motor_group.set_velocity(100).unwrap();
    sleep(Duration::from_secs(1)).await;

    // Brake the motor group
    motor_group.brake(BrakeMode::Hold).unwrap();
}
