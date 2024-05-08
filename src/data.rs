use std::ops::{AddAssign, ShlAssign, ShrAssign, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Data {
    pointer: usize,
    data: Vec<u8>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Data {
            pointer: 0,
            data: vec![0; 30_000],
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.data[self.pointer]
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[self.pointer] as char)
    }
}

impl AddAssign<i32> for Data {
    fn add_assign(&mut self, rhs: i32) {
        self.data[self.pointer] += rhs as u8;
    }
}

impl SubAssign<i32> for Data {
    fn sub_assign(&mut self, rhs: i32) {
        self.data[self.pointer] -= rhs as u8;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<i32> for Data {
    fn shl_assign(&mut self, rhs: i32) {
        self.pointer -= rhs as usize;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShrAssign<i32> for Data {
    fn shr_assign(&mut self, rhs: i32) {
        self.pointer += rhs as usize;
    }
}

impl PartialEq<i32> for Data {
    fn eq(&self, other: &i32) -> bool {
        self.data[self.pointer] == *other as u8
    }
}
