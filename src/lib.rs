//! # vexide-motorgroup
//!
//! Missing `MotorGroup` from VEXCode or PROS? This is a simple implementation of a
//! `MotorGroup` for vexide which allows you to group motors together and control
//! them as one.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! // ... other dependencies
//! vexide-motorgroup = "1.0.0"
//! ```
//!
//! Or if you prefer the command line:
//!
//! ```sh
//! cargo add vexide-motorgroup
//! ```
//!
//! ## Usage
//!
//! Normally, you would have to set each motor's target and other values
//! individually even if the motors were physically connected in a drivetrain or
//! similar, but with `MotorGroup`, you can control them as if they were one motor.
//!
//! Just create a `MotorGroup` with a `Vec` of `Motor`s and use the `MotorGroup`
//! methods just like you would with a `Motor`. It's that simple!
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! extern crate alloc;
//!
//! use core::time::Duration;
//!
//! use alloc::vec;
//! use vexide_motorgroup::MotorGroup;
//!
//! use vexide::prelude::*;
//!
//! #[vexide::main]
//! async fn main(peripherals: Peripherals) {
//!     // Here's where the magic happens
//!     let mut motor_group = MotorGroup::new(vec![
//!         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
//!         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
//!     ]);
//!
//!     // Set the motor group's target to a voltage as if it were a motor
//!     motor_group.set_voltage(5.0).unwrap();
//!     sleep(Duration::from_secs(1)).await;
//!
//!     // Set the motor group's target to a position
//!     motor_group
//!         .set_position_target(Position::from_degrees(90.0), 200)
//!         .unwrap();
//!     sleep(Duration::from_secs(1)).await;
//!
//!     // Set the motor group's target to a velocity
//!     motor_group.set_velocity(100).unwrap();
//!     sleep(Duration::from_secs(1)).await;
//!
//!     // Brake the motor group
//!     motor_group.brake(BrakeMode::Hold).unwrap();
//! }
//! ```

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use vexide::{
    devices::smart::{motor::MotorError, Motor},
    prelude::{BrakeMode, Direction, Gearset, MotorControl, Position},
};

/// A group of motors that can be controlled together.
///
/// This is a simple wrapper around a vector of motors, with methods to easily
/// control all motors in the group at once as if they were a single motor.
///
/// A motor group is guaranteed to have at least one motor in it.
#[derive(Debug)]
pub struct MotorGroup(Vec<Motor>);

/// An error that occurs when controlling a motor group.
///
/// This error is returned when an individual motor in the group encounters an
/// error. The error contains a list of all the errors that occurred.
///
/// A MotorGroupError is guaranteed to have at least one error in it.
///
/// MotorGroupError also implements `Into<MotorError>`, which will return the
/// first error that occurred. This means that you can use the `?` operator
/// with a `MotorGroupError` to return a `MotorError` to a result.
#[derive(Debug)]
#[non_exhaustive]
pub struct MotorGroupError {
    pub errors: Vec<MotorError>,
}

impl MotorGroupError {
    /// Creates a new motor group error from a `Vec` of motor errors.
    ///
    /// # Panics
    ///
    /// Panics if the errors vector is empty.
    pub(crate) fn new(errors: Vec<MotorError>) -> Self {
        if errors.is_empty() {
            panic!("Cannot create a MotorGroupError with no errors");
        }
        Self { errors }
    }

    /// The first error that occurred in the motor group.
    pub fn first(&self) -> &MotorError {
        &self.errors[0]
    }

    /// Whether the motor group has a busy error.
    ///
    /// A busy error occurs when communication with a motor is not possible
    /// when reading flags.
    pub fn has_busy_error(&self) -> bool {
        self.errors
            .iter()
            .any(|error| matches!(error, MotorError::Busy))
    }

    /// Whether the motor group has a port error.
    ///
    /// A port error occurs when a motor is not currently connected to a Smart
    /// Port.
    pub fn has_port_error(&self) -> bool {
        self.errors
            .iter()
            .any(|error| matches!(error, MotorError::Port { source: _ }))
    }
}

impl From<MotorGroupError> for MotorError {
    fn from(error: MotorGroupError) -> Self {
        error.errors.into_iter().next().unwrap()
    }
}

impl core::fmt::Display for MotorGroupError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "error(s) in MotorGroup: {:?}", self.errors)
    }
}

impl core::error::Error for MotorGroupError {}

impl MotorGroup {
    /// Creates a new motor group from a vector of motors.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     _ = motor_group.set_voltage(5.0);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if there are no motors in the vector.
    pub fn new(motors: Vec<Motor>) -> Self {
        if motors.is_empty() {
            panic!("Cannot create a motor group with no motors");
        }
        Self(motors)
    }

    /// Sets the target that the motor group should attempt to reach.
    ///
    /// This could be a voltage, velocity, position, or even brake mode.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let _ = motor_group.set_target(MotorControl::Voltage(5.0));
    ///     sleep(Duration::from_secs(1)).await;
    ///     let _ = motor_group.set_target(MotorControl::Brake(BrakeMode::Hold));
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_target).
    pub fn set_target(&mut self, target: MotorControl) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_target(target) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's target to a given [`BrakeMode`].
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let _ = motor_group.brake(BrakeMode::Hold);
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.brake).
    pub fn brake(&mut self, mode: BrakeMode) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.brake(mode) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Spins the motor group at a target velocity.
    ///
    /// This velocity corresponds to different actual speeds in RPM depending on the gearset used for the motor.
    /// Velocity is held with an internal PID controller to ensure consistent speed, as opposed to setting the
    /// motor's voltage.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Spin a motor group at 100 RPM:
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let _ = motor_group.set_velocity(100);
    ///     sleep(Duration::from_secs(1)).await;
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_velocity).
    pub fn set_velocity(&mut self, rpm: i32) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_velocity(rpm) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's output voltage.
    ///
    /// This voltage value spans from -12 (fully spinning reverse) to +12 (fully spinning forwards) volts, and
    /// controls the raw output of the motor.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Give the motor group full power:
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let _ = motor_group.set_voltage(motor_group.max_voltage());
    /// }
    /// ```
    ///
    /// Drive the motor group based on a controller joystick:
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let controller = peripherals.primary_controller;
    ///     loop {
    ///         let controller_state = controller.state().unwrap_or_default();
    ///         let voltage = controller_state.left_stick.x() * motor_group.max_voltage();
    ///         motor_group.set_voltage(voltage).unwrap();
    ///     }
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_voltage).
    pub fn set_voltage(&mut self, volts: f64) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_voltage(volts) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets an absolute position target for the motor group to attempt to reach.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    ///
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     let _ = motor_group.set_position_target(Position::from_degrees(90.0), 200);
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_position_target).
    pub fn set_position_target(
        &mut self,
        position: Position,
        velocity: i32,
    ) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_position_target(position, velocity) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Changes the output velocity for a profiled movement (motor_move_absolute or motor_move_relative).
    ///
    /// This will have no effect if the motor group is not following a profiled movement.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///     // Set the motor group's target to a Position so that changing the velocity isn't a noop.
    ///     let _ = motor_group.set_target(MotorControl::Position(Position::from_degrees(90.0), 200));
    ///     let _ = motor_group.set_profiled_velocity(100).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_profiled_velocity).
    pub fn set_profiled_velocity(&mut self, velocity: i32) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_profiled_velocity(velocity) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the gearset of an 11W motor group.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    /// - A [`MotorError::SetGearsetExp`] is returned if the motor is a 5.5W EXP Smart Motor, which has no swappable gearset.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     // This must be a V5 motor group
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     // Set the motor group to use the red gearset
    ///     motor_group.set_gearset(Gearset::Red).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_gearset).
    pub fn set_gearset(&mut self, gearset: Gearset) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_gearset(gearset) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns `true` if the motor group has a 5.5W (EXP) Smart Motor.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new_exp(peripherals.port_1, Direction::Forward),
    ///         Motor::new_exp(peripherals.port_2, Direction::Forward),
    ///     ]);
    ///     if motor_group.has_exp() {
    ///         println!("Motor group has a 5.5W EXP Smart Motor");
    ///     }
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_exp).
    pub fn has_exp(&self) -> bool {
        self.0.iter().any(|motor| motor.is_exp())
    }

    /// Returns `true` if the motor group has an 11W (V5) Smart Motor.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Red, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Red, Direction::Forward),
    ///     ]);
    ///     if motor_group.has_v5() {
    ///         println!("Motor group has an 11W V5 Smart Motor");
    ///     }
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_v5).
    pub fn has_v5(&self) -> bool {
        self.0.iter().any(|motor| motor.is_v5())
    }

    /// Returns the maximum voltage for the motor group based off of its [motor type](Motor::motor_type).
    ///
    /// # Examples
    ///
    /// Run a motor group at max speed, agnostic of its type:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// fn run_motor_group_at_max_speed(motor_group: &mut MotorGroup) {
    ///     motor_group.set_voltage(motor_group.max_voltage()).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.max_voltage).
    pub fn max_voltage(&self) -> f64 {
        self.0
            .iter()
            .map(|motor| motor.max_voltage())
            .reduce(f64::max)
            .unwrap()
    }

    /// Returns the average estimated angular velocity of motors in a motor group in rotations per minute (RPM).
    ///
    /// # Accuracy
    ///
    /// In some cases, this reported value may be noisy or innaccurate, especially for systems where accurate
    /// velocity control at high speeds is required (such as flywheels). If the accuracy of this value proves
    /// inadequate, you may opt to perform your own velocity calculations by differentiating [`Motor::position`]
    /// over the reported internal timestamp of the motor using [`Motor::timestamp`].
    ///
    /// > For more information about Smart motor velocity estimation, see [this article](https://sylvie.fyi/sylib/docs/db/d8e/md_module_writeups__velocity__estimation.html).
    ///
    /// # Note
    ///
    /// To get the current **target** velocity instead of the estimated velocity, use [`Motor::target`].
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Get the current velocity of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.velocity().unwrap());
    /// }
    /// ```
    ///
    /// Calculate acceleration of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     let mut last_velocity = motor_group.velocity().unwrap();
    ///     let mut start_time = Instant::now();
    ///     loop {
    ///         let velocity = motor_group.velocity().unwrap();
    ///         // Make sure we don't divide by zero
    ///         let elapsed = start_time.elapsed().as_secs_f64() + 0.001;
    ///
    ///         // Calculate acceleration
    ///         let acceleration = (velocity - last_velocity) / elapsed;
    ///         println!("Velocity: {:.2} RPM, Acceleration: {:.2} RPM/s", velocity, acceleration);
    ///
    ///         last_velocity = velocity;
    ///         start_time = Instant::now();
    ///    }
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.velocity).
    pub fn velocity(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.velocity() {
                Ok(velocity) => {
                    sum += velocity;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the average power drawn by a motor in this the motor group in Watts.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the power drawn by a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.power().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.power).
    pub fn power(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.power() {
                Ok(power) => {
                    sum += power;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the average torque of motors in the motor group in Newton-meters.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the torque of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.torque().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.torque).
    pub fn torque(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.torque() {
                Ok(torque) => {
                    sum += torque;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the motor group's output voltage.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the voltage of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.voltage().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.voltage).
    pub fn voltage(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.voltage() {
                Ok(voltage) => {
                    sum += voltage;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the motor group's average position in ticks.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the position of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.position().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.position).
    pub fn position(&self) -> Result<Position, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = Position::from_ticks(0, 36000);
        let mut count = 0;
        for motor in &self.0 {
            match motor.position() {
                Ok(position) => {
                    sum += position;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as i64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the motor group's average current in Amperes.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the current of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.current().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.current).
    pub fn current(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.current() {
                Ok(current) => {
                    sum += current;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the motor group's average efficiency as a percentage.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the efficiency of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.efficiency().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.efficiency).
    pub fn efficiency(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.efficiency() {
                Ok(efficiency) => {
                    sum += efficiency;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Resets every motor in the motor group's position to zero.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Reset the position of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     motor_group.reset_position().unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.reset_position).
    pub fn reset_position(&mut self) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.reset_position() {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's position to a given value.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Set the position of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     motor_group.set_position(Position::from_degrees(90.0)).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_position).
    pub fn set_position(&mut self, position: Position) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_position(position) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's current limit in Amperes.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Set the current limit of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     motor_group.set_current_limit(2.5).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_current_limit).
    pub fn set_current_limit(&mut self, limit: f64) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_current_limit(limit) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's voltage limit in Volts.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Set the voltage limit of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     motor_group.set_voltage_limit(10.0).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_voltage_limit).
    pub fn set_voltage_limit(&mut self, limit: f64) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_voltage_limit(limit) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns the motor group's temperature in degrees Celsius.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor in the group encounters an error.
    ///
    /// # Examples
    ///
    /// Print the temperature of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.temperature().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.temperature).
    pub fn temperature(&self) -> Result<f64, MotorGroupError> {
        let mut errors = Vec::new();
        let mut sum = 0.0;
        let mut count = 0;
        for motor in &self.0 {
            match motor.temperature() {
                Ok(temperature) => {
                    sum += temperature;
                    count += 1;
                }
                Err(error) => errors.push(error),
            }
        }
        if errors.is_empty() {
            Ok(sum / count as f64)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns `true` if any motor in the motor group is over temperature.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor encounters an error.
    ///
    /// # Examples
    ///
    /// Check if a motor group is over temperature:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.is_over_temperature().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_over_temperature).
    pub fn is_over_temperature(&self) -> Result<bool, MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &self.0 {
            match motor.is_over_temperature() {
                Ok(true) => return Ok(true),
                Err(error) => errors.push(error),
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(false)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns `true` if any motor in the motor group is over current.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor encounters an error.
    ///
    /// # Examples
    ///
    /// Check if a motor group is over current:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.is_over_current().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_over_current).
    pub fn is_over_current(&self) -> Result<bool, MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &self.0 {
            match motor.is_over_current() {
                Ok(true) => return Ok(true),
                Err(error) => errors.push(error),
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(false)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns `true` if any motor in the motor group has a driver fault.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor encounters an error.
    ///
    /// # Examples
    ///
    /// Check if a motor group has a driver fault:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.is_driver_fault().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_driver_fault).
    pub fn is_driver_fault(&self) -> Result<bool, MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &self.0 {
            match motor.is_driver_fault() {
                Ok(true) => return Ok(true),
                Err(error) => errors.push(error),
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(false)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Returns `true` if the any motor in the motor group is over current.
    ///
    /// # Errors
    ///
    /// - A [`MotorGroupError`] error is returned if any motor encounters an error.
    ///
    /// # Examples
    ///
    /// Check if a motor group is over current:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     println!("{:?}", motor_group.is_driver_over_current().unwrap());
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.is_driver_over_current).
    pub fn is_driver_over_current(&self) -> Result<bool, MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &self.0 {
            match motor.is_driver_over_current() {
                Ok(true) => return Ok(true),
                Err(error) => errors.push(error),
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(false)
        } else {
            Err(MotorGroupError::new(errors))
        }
    }

    /// Sets the motor group's direction.
    ///
    /// # Errors
    ///
    /// - A [`MotorError::Port`] error is returned if a motor device is not currently connected to the Smart Port.
    ///
    /// # Examples
    ///
    /// Set the direction of a motor group:
    /// ```
    /// use vexide::prelude::*;
    ///
    /// #[vexide::main]
    /// async fn main(peripherals: Peripherals) {
    ///     let mut motor_group = MotorGroup::new(vec![
    ///         Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
    ///         Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
    ///     ]);
    ///
    ///     motor_group.set_direction(Direction::Reverse).unwrap();
    /// }
    /// ```
    ///
    /// See the original method [here](https://docs.rs/vexide/latest/vexide/devices/smart/struct.Motor.html#method.set_direction).
    pub fn set_direction(&mut self, direction: Direction) -> Result<(), MotorGroupError> {
        let mut errors = Vec::new();
        for motor in &mut self.0 {
            if let Err(error) = motor.set_direction(direction) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(MotorGroupError::new(errors))
        }
    }
}
