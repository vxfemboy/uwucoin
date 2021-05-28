mod blockchain;
use crate::blockchain::block::Blk;
use crate::blockchain::hash::*;



fn main() {
    println!("blockchain test\n\n");

    let mut block = Blk::new(0,0,vec![0, 32], 0, "Gen blk".to_owned());
    println!("{:?}", &block); //change debug to mainstream
    let uwuh = block.hash();

    
    println!("{:?}", &uwuh);
    block.hash = uwuh;

    println!("{:?}", &block);

}
