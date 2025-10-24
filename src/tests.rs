// Unit tests for `lib.rs` covering MotorGroupError helpers and error strategy
// These tests avoid hardware-specific APIs and focus on pure-data helpers

#![cfg(test)]

use crate::{MotorGroupError, WriteErrorStrategy};

#[derive(Debug, PartialEq, Eq, Clone)]
struct FakeErr(&'static str);

#[test]
fn motor_group_error_new_panics_on_empty() {
    // MotorGroupError::new should panic when given an empty errors vector
    let v: Vec<FakeErr> = vec![];
    let res = std::panic::catch_unwind(|| MotorGroupError::new(v));
    assert!(
        res.is_err(),
        "expected MotorGroupError::new to panic on empty errors vector"
    );
}

#[test]
fn motor_group_error_with_result_and_first() {
    let errors = vec![FakeErr("a"), FakeErr("b")];

    // with_result is crate-private but available to tests in the same crate
    let mg_err = MotorGroupError::with_result(errors.clone(), 42u32);

    // first() returns a reference to the first error
    assert_eq!(mg_err.first(), &errors[0]);

    // result() should return the provided result
    assert_eq!(mg_err.result(), &Some(42u32));

    // errors vector should be preserved
    assert_eq!(mg_err.errors.len(), 2);
}

#[test]
fn motor_group_error_with_empty_result() {
    let errors = vec![FakeErr("x")];

    let mg_err: MotorGroupError<FakeErr, ()> = MotorGroupError::with_empty_result(errors.clone());

    // No result available when using with_empty_result
    assert_eq!(mg_err.result(), &None);

    // Error list is present and first() works
    assert_eq!(mg_err.errors.len(), 1);
    assert_eq!(mg_err.first(), &errors[0]);
}

#[test]
fn write_error_strategy_default_is_ignore() {
    // Default strategy should be Ignore
    assert_eq!(WriteErrorStrategy::default(), WriteErrorStrategy::Ignore);
}
