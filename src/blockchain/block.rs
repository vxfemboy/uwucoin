use std::fmt::{ self, Debug, Formatter };
use crate::BlkHash;

pub struct Blk {
    pub index: u32,
    pub timestampt: u128,
    pub hash: BlkHash,
    pub prev_blk: BlkHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Blk {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "blk")
    }
}

impl Blk {


}