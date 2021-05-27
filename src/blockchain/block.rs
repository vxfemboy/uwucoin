use std::fmt::{ self, Debug, Formatter };

pub struct Blk {
}

impl Debug for Blk {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "blk")
    }
}

impl Blk {
}