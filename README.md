# vexide-motorgroup

[![Documentation](https://docs.rs/vexide-motorgroup/badge.svg)](https://docs.rs/vexide-motorgroup)
[![Crates.io](https://img.shields.io/crates/v/vexide-motorgroup.svg)](https://crates.io/crates/vexide-motorgroup)
[![License](https://img.shields.io/crates/l/vexide-motorgroup.svg)](https://github.com/zabackary/vexide-motorgroup/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/vexide-motorgroup.svg)](https://crates.io/crates/vexide-motorgroup)
[![CI status](https://github.com/zabackary/vexide-motorgroup/actions/workflows/build.yml/badge.svg)](https://github.com/zabackary/vexide-motorgroup/actions/workflows/build.yml)
[![Made with vexide badge](https://img.shields.io/badge/Made%20with-vexide-e6c85c.svg?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAMAAABEpIrGAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAAHmUExURQAAAOXIW+bIXObIXNfb6dfb6djb6djb6eXIXOXIXObIXObIXObIXOXIXNjb6djb6djb6djb6eXIW+XIW+bIXObIW+bIW+bIW+bHW9jb6djb6djb6Njb6OXHXObIXObIXObHXObIW+XIW+XIW+bHXObHW9jb6Njb6dja6Nfb6uXHXOXHXObIXObIXObIXOXIW+bIW+bHXObHW+bIW9fb79jb6Njb6djb6djb6Njb6eXIXOXHXObIXObIXObHXObHW+bHXObIW+bIW9fb6djb6dja6Nfa6eXHXOXIW+bIW+bIW+bHW+bHXObHXObHXObIXObIXObIW+bHW+bIW+bHW+bHXObIW+bIXObIXOXHXOXHW+bHXOXIXOXHXOXHXObHXObHXObIW+XHW+bHXObIW+XHXObHXOXIXObIXObHW+bHW+XIXOXIXObIXObIW+bIW+bIXObIW+bIXObIW+bIW+bIXObIW+bIXOXIXObIXOXIW+bIW+bIW+bIW+bIXObHXOXHXOXHXObHXObIXObIXOXIW+XIW+XIXOXHXOXHXObHXObHXObHXObIXObIXObIXObIXObHXObIW+bIW+XHXObHXOXHXObIXObIXObIXObIXObIXObIW+bIXObHXObIXNjb6QAAAKYhk3UAAACfdFJOUwDh5EMBmudIQ+Fs7NM3PtzYMzbTAmLv0TRG4u1GNNHvAwNn8c4xUO5hAjDO8WcDBGvyyy4BWe3xZQMty/IEBHD0yCsDVloEKsj0cAQFdPXEKCjEdAUGePbBJSWYB333vSEhvcUlCIH5yL0jCYX6uiAKivjVth4Mjvv7jhmJ+rIbDZL8/JINC437rRwOl/2XDv66EJqaEBBui4p7Fa6uFUfmhQMAAAABYktHRACIBR1IAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAB3RJTUUH6QEaBTUsaXkSHwAAAURJREFUOMtjYBgZgHE+CDAxMzCwsC4AATZ2sDgHE1iCk4GLG8zg4WVg4OMHqxAQBMoLCYOFubkYRETFwExxCQYGSSmQAmkZBgZZObCgvKgCA4OikjKYo6LKwKDGtmCBuoYmg5Y2WEhHVw9km76BIZhrZMxgYmpmbmHJYGUNFrDhsoW4087eASzg6MTg7OLqxuDuAeZ6ennDfOLj6wcW8g8A8QKDwBy/4BCEX0PDwsGCEZEMDFERYOb8aB/k0IiJjQOLxickJkHk5yenoIRXalo6WDgjYz4MZGahqMjOyZ2PBvLyUVQUFMJVFBUXgemSUhQVZeUVEPnKquqaWjCrrh5FRUNjE0i0uaWVoa29A6yiswtFRXdP7/z5vX39YMXNIAUTJqLG/aTJvb1TpoKZ06bPmDlz5oxZaKlj9py58wY0eQ5aAACvpKcarQF0GAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAAASUVORK5CYII=)](https://github.com/doxa-robotics)

Missing `MotorGroup` from VEXCode or PROS? This is a simple implementation of a
`MotorGroup` for vexide which allows you to group motors together and control
them as one.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
# ... other dependencies
vexide-motorgroup = "2.1.0"
```

Or if you prefer the command line:

```sh
cargo add vexide-motorgroup
```

## Usage

Normally, you would have to set each motor's target and other values
individually even if the motors were physically connected in a drivetrain or
similar, but with `MotorGroup`, you can control them as if they were one motor.

Just create a `MotorGroup` with a `Vec` of `Motor`s and use the `MotorGroup`
methods just like you would with a `Motor`. It's that simple!

```rust
#![no_std]
#![no_main]

extern crate alloc;

use core::time::Duration;

use alloc::vec;
use vexide_motorgroup::MotorGroup;

use vexide::prelude::*;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    // Here's where the magic happens
    let mut motor_group = MotorGroup::new(vec![
        Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ]);

    // Set the motor group's target to a voltage as if it were a motor
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
```

## Error handling

### Read errors

For functions returning values and reading data (i.e., those taking a read-only
reference to self), upon encountering an error accessing any motor, the result
will be a MotorGroupError that contains all the errors encountered during the
operation. Using
[`MotorGroupError::result`](https://docs.rs/vexide-motorgroup/latest/vexide_motorgroup/struct.MotorGroupError.html#method.result)
will return the average of all the results that were successfully read.

### Write errors

vexide-motorgroup provides two different strategies for handling write errors.
Both of them will return an `Err` when any motor returns an error.

1. [`WriteErrorStrategy::Ignore`](https://docs.rs/vexide-motorgroup/latest/vexide_motorgroup/enum.WriteErrorStrategy.html#variant.Ignore)
   (default): This strategy will ignore errors and continue writing to the other
   motors.
2. [`WriteErrorStrategy::Stop`](https://docs.rs/vexide-motorgroup/latest/vexide_motorgroup/enum.WriteErrorStrategy.html#variant.Stop):
   This strategy will stop writing to the other motors and return the error
   immediately.
