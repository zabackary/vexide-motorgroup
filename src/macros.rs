/// A macro that creates a set of sharable motors.
///
/// See [`motor_group!`] for more details.
/// ```
#[macro_export]
macro_rules! shared_motors {
    ( $item:tt ) => {{ $crate::shared_motors::SharedMotors::new($crate::motor_group!($item)) }};
}

/// A macro that creates a set of motors using [`MotorGroup`](crate::MotorGroup).
///
/// This macro uses a similar syntax to the [`vec!`] macro, but it creates a
/// [`MotorGroup`](crate::MotorGroup) instead of a vector.
#[macro_export]
macro_rules! motor_group {
    ( $( $item:expr ),* $(,)?) => {
        {
            use ::core::cell::RefCell;
            use ::alloc::{vec::Vec};

            let mut temp_vec: Vec<Motor> = Vec::new();

            $(
                temp_vec.push($item);
            )*

            $crate::MotorGroup::new(temp_vec)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::MotorGroup;

    use super::*;
    use vexide::{prelude::*, smart::SmartPort};

    #[test]
    fn motor_group_compiles() {
        let mg_1 = motor_group![
            Motor::new(
                unsafe { SmartPort::new(2) },
                Gearset::Green,
                Direction::Forward
            ),
            Motor::new(
                unsafe { SmartPort::new(1) },
                Gearset::Green,
                Direction::Forward
            ),
        ];
    }
}

// TODO: Add a macro for creating a motor group using only a list of ports and
//       a gearset. It would do something like PROS does with - indicating
//       reversed motors.
