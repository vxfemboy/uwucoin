mod blockchain;
use crate::blockchain::block::Blk;
use crate::blockchain::hash::*;



fn main() {
    println!("blockchain test\n\n");
    let block = Blk::new(0,0,vec![0, 32], 0, "Gen blk".to_owned());
    println!("{:?}", &block); //change debug to mainstream
    let uwu = block.hash();

    println!("{:?}", &uwu)
}
