use cucumber::{given, when, then, World};
use std::convert::Infallible;

#[derive(Debug, Default, World)]
pub struct MathWorld {
    number: i32,
}

impl cucumber::WorldInit for MathWorld {
    type Error = Infallible;
}

#[given(regex = r"I have the number (\d+)")]
async fn set_number(w: &mut MathWorld, num: i32) {
    w.number = num;
}

#[when(regex = r"I add (\d+)")]
async fn add_number(w: &mut MathWorld, num: i32) {
    w.number += num;
}

#[then(regex = r"the result should be (\d+)")]
async fn check_result(w: &mut MathWorld, expected: i32) {
    assert_eq!(w.number, expected);
}
