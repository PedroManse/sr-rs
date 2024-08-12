pub use openssl::error::ErrorStack as Error;
use openssl::symm::{self, Cipher};

pub fn encrypt<P1, P2>(data: P1, key: P2) -> Result<Vec<u8>, Error>
where
    P1: AsRef<[u8]>,
    P2: AsRef<[u8]>
{
    let key_hash = crate::hash(key);
    let cipher = Cipher::aes_256_cbc();
    symm::encrypt(cipher, &key_hash, None, data.as_ref())
}

pub fn decrypt<P1, P2>(data: P1, key: P2) -> Result<Vec<u8>, Error>
where
    P1: AsRef<[u8]>,
    P2: AsRef<[u8]>
{
    let key_hash = crate::hash(key);
    let cipher = Cipher::aes_256_cbc();
    symm::decrypt(cipher, &key_hash, None, data.as_ref())
}

