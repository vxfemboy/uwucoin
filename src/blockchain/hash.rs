pub trait Hshb {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Vec<u8> {
        //implement custom hash function here 
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
} 