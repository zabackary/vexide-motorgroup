use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};
use vexide::{
    math::Angle,
    prelude::*,
    smart::motor::{BrakeMode, MotorControl, SetGearsetError},
};

use crate::{GetterResult, MotorGroup, MotorGroupError, WriteErrorStrategy};

/// Motors that can be cloned with interior mutability.
///
/// This simply wraps MotorGroups with a newtype while adding some traits useful
/// for using them.
#[derive(Clone, Debug)]
pub struct SharedMotors<M: AsRef<[Motor]> + AsMut<[Motor]> = Vec<Motor>>(
    pub Rc<RefCell<MotorGroup<M>>>,
);

impl<M: AsRef<[Motor]> + AsMut<[Motor]>> SharedMotors<M> {
    /// Create a new SharedMotors from a MotorGroup.
    pub fn new(motors: MotorGroup<M>) -> Self {
        Self(Rc::new(RefCell::new(motors)))
    }

    /// See [`MotorGroup::write_error_strategy`].
    pub fn write_error_strategy(&mut self, mode: WriteErrorStrategy) -> &Self {
        self.0.borrow_mut().write_error_strategy(mode);
        self
    }

    /// See [`MotorGroup::set_target`].
    pub fn set_target(&mut self, target: MotorControl) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_target(target)
    }

    /// See [`MotorGroup::brake`].
    pub fn brake(&mut self, mode: BrakeMode) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().brake(mode)
    }

    /// See [`MotorGroup::set_velocity`].
    pub fn set_velocity(&mut self, rpm: i32) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_velocity(rpm)
    }

    /// See [`MotorGroup::set_voltage`].
    pub fn set_voltage(&mut self, volts: f64) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_voltage(volts)
    }

    /// See [`MotorGroup::set_position_target`].
    pub fn set_position_target(
        &mut self,
        position: Angle,
        velocity: i32,
    ) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_position_target(position, velocity)
    }

    /// See [`MotorGroup::set_profiled_velocity`].
    pub fn set_profiled_velocity(&mut self, velocity: i32) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_profiled_velocity(velocity)
    }

    /// See [`MotorGroup::set_gearset`].
    pub fn set_gearset(
        &mut self,
        gearset: Gearset,
    ) -> Result<(), MotorGroupError<SetGearsetError>> {
        self.0.borrow_mut().set_gearset(gearset)
    }

    /// See [`MotorGroup::has_exp`].
    pub fn has_exp(&self) -> bool {
        self.0.borrow().has_exp()
    }

    /// See [`MotorGroup::has_v5`].
    pub fn has_v5(&self) -> bool {
        self.0.borrow().has_v5()
    }

    /// See [`MotorGroup::max_voltage`].
    pub fn max_voltage(&self) -> f64 {
        self.0.borrow().max_voltage()
    }

    /// See [`MotorGroup::velocity`].
    pub fn velocity(&self) -> GetterResult<f64> {
        self.0.borrow().velocity()
    }

    /// See [`MotorGroup::power`].
    pub fn power(&self) -> GetterResult<f64> {
        self.0.borrow().power()
    }

    /// See [`MotorGroup::torque`].
    pub fn torque(&self) -> GetterResult<f64> {
        self.0.borrow().torque()
    }

    /// See [`MotorGroup::voltage`].
    pub fn voltage(&self) -> GetterResult<f64> {
        self.0.borrow().voltage()
    }

    /// See [`MotorGroup::position`].
    pub fn position(&self) -> GetterResult<Angle> {
        self.0.borrow().position()
    }

    /// See [`MotorGroup::current`].
    pub fn current(&self) -> GetterResult<f64> {
        self.0.borrow().current()
    }

    /// See [`MotorGroup::efficiency`].
    pub fn efficiency(&self) -> GetterResult<f64> {
        self.0.borrow().efficiency()
    }

    /// See [`MotorGroup::reset_position`].
    pub fn reset_position(&self) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().reset_position()
    }

    /// See [`MotorGroup::set_position`].
    pub fn set_position(&mut self, position: Angle) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_position(position)
    }

    /// See [`MotorGroup::set_current_limit`].
    pub fn set_current_limit(&mut self, limit: f64) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_current_limit(limit)
    }

    /// See [`MotorGroup::set_voltage_limit`].
    pub fn set_voltage_limit(&mut self, limit: f64) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_voltage_limit(limit)
    }

    /// See [`MotorGroup::temperature`].
    pub fn temperature(&self) -> GetterResult<f64> {
        self.0.borrow().temperature()
    }

    /// See [`MotorGroup::is_over_temperature`].
    pub fn is_over_temperature(&self) -> Result<bool, MotorGroupError> {
        self.0.borrow().is_over_temperature()
    }

    /// See [`MotorGroup::is_over_current`].
    pub fn is_over_current(&self) -> Result<bool, MotorGroupError> {
        self.0.borrow().is_over_current()
    }

    /// See [`MotorGroup::is_driver_fault`].
    pub fn is_driver_fault(&self) -> Result<bool, MotorGroupError> {
        self.0.borrow().is_driver_fault()
    }

    /// See [`MotorGroup::is_driver_over_current`].
    pub fn is_driver_over_current(&self) -> Result<bool, MotorGroupError> {
        self.0.borrow().is_driver_over_current()
    }

    /// See [`MotorGroup::set_direction`].
    pub fn set_direction(&mut self, direction: Direction) -> Result<(), MotorGroupError> {
        self.0.borrow_mut().set_direction(direction)
    }
}
