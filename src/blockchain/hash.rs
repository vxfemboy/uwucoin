pub trait Hshb {
    fn bytes (self) -> Vec<u8>;

    fn hash (&self) -> Vec<u8> {
        //implement hash function here 
    }
} 