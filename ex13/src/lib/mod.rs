use std::fmt;
use std::ops;

pub mod compute;
pub mod parser;

#[derive(Clone, Debug)]
pub struct Timestamp(usize);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Self) -> Self::Output {
        Timestamp(self.0 + rhs.0)
    }
}

impl ops::Sub for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: Self) -> Self::Output {
        Timestamp(self.0 - rhs.0)
    }
}

impl Timestamp {
    pub fn get_value(self) -> usize {
        self.0
    }
}

pub struct BusID(usize);

impl fmt::Display for BusID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl BusID {
    pub fn get_value(self) -> usize {
        self.0
    }
}

pub type BusIDs = Vec<BusID>;
