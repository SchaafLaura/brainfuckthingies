use std::ops::{AddAssign, ShlAssign, ShrAssign, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Data {
    pointer: i32,
    data: Vec<u8>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Data {
            pointer: 0,
            data: vec![0; 30_000],
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[self.pointer as usize] as char)
    }
}

impl AddAssign<i32> for Data {
    fn add_assign(&mut self, rhs: i32) {
        self.pointer += rhs;
    }
}

impl SubAssign<i32> for Data {
    fn sub_assign(&mut self, rhs: i32) {
        self.pointer -= rhs;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<u8> for Data {
    fn shl_assign(&mut self, rhs: u8) {
        self.data[self.pointer as usize] -= rhs;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShrAssign<u8> for Data {
    fn shr_assign(&mut self, rhs: u8) {
        self.data[self.pointer as usize] += rhs;
    }
}
