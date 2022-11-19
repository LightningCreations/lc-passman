use lc_crypto::digest::{Hmac, sha2::Sha256, digest};


pub fn derive_key(k: &[u8], c: &[u8], out: &mut [u8]){
    let alg = Hmac::new(Sha256::new(), k);

    digest(alg, c, out);
}