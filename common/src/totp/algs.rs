use crate::time::Timespec;


pub trait TotpAlgorithm{
    fn get_password(&self, timespec: Timespec, secret: &[u8], outbuf: &mut [u8]) -> std::io::Result<()>;
    fn algname(&self) -> &'static str;
}


macro_rules! totp_hmac_algorithm{
    [$(($name:ident, $algname:literal, $digest_alg:ty)),* $(,)?] => {
        $(pub struct $name;

        impl TotpAlgorithm for $name{
            fn get_password(&self, timespec: Timespec, secret: &[u8], outbuf: &mut [u8]) -> std::io::Result<()>{
                let counter = timespec.keytime().to_be_bytes();

                let hash = lc_crypto::digest::Hmac::new(<$digest_alg>::new(),secret);

                let mut buf = [0u8;<$digest_alg as lc_crypto::digest::Digest>::OUTPUT_SIZE];

                lc_crypto::digest::digest(hash,&counter,&mut buf);

                outbuf.copy_from_slice(&buf[..outbuf.len()]);

                Ok(())
            }
            fn algname(&self) -> &'static str{
                $algname
            }
        })*
    }
}

totp_hmac_algorithm![
    (TotpAlgSha1, "sha1", lc_crypto::digest::sha1::Sha1),
    (TotpAlgSha256, "sha256", lc_crypto::digest::sha2::Sha256),
    (TotpAlgSha512, "sha512", lc_crypto::digest::sha2::Sha512)
];


static TOTP_ALGS: &[&(dyn TotpAlgorithm+Sync+'static)] = &[
    &TotpAlgSha1,
    &TotpAlgSha256,
    &TotpAlgSha512
];

pub fn algorithm_by_name(name: &str) -> Option<&'static dyn TotpAlgorithm>{
    for alg in TOTP_ALGS{
        if alg.algname()==name{
            return Some(*alg);
        }
    }

    None
}