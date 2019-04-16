//! Foreign function interfaces
//!
//! This module defines the foreign function interface for the following
//! crypto implementations from PQClean:
//!
//!  * sphincs-shake256-128f-simple
// This file has been generated from PQClean.
// Find the templates in pqcrypto-template
use libc::c_int;

pub const PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_SECRETKEYBYTES: usize = 64;
pub const PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_PUBLICKEYBYTES: usize = 32;
pub const PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_BYTES: usize = 16976;

#[link(name = "sphincsplus")]
extern "C" {
    pub fn PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign(sm: *mut u8, smlen: *mut usize, msg: *const u8, len: usize, sk: *const u8) -> c_int;
    pub fn PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_open(m: *mut u8, mlen: *mut usize, sm: *const u8, smlen: usize, pk: *const u8) -> c_int;
    pub fn PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_signature(sig: *mut u8, siglen: *mut usize, m: *const u8, mlen: usize, sk: *const u8) -> c_int;
    pub fn PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_verify(sig: *const u8, siglen: usize, m: *const u8, mlen: usize, pk: *const u8) -> c_int;
}

#[cfg(test)]
mod test_sphincsshake256128fsimple_clean {
    use super::*;
    use std::mem;
    use rand::prelude::*;


    #[test]
    fn test_ffi() {
        unsafe {
            let mut rng = rand::thread_rng();
            let mut mlen: usize = rng.gen::<u16>() as usize;
            let msg: Vec<u8> = (0..mlen).map(|_| rng.gen()).collect();

            let mut pk: [u8; PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_PUBLICKEYBYTES] = mem::uninitialized();
            let mut sk: [u8; PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_SECRETKEYBYTES] = mem::uninitialized();
            let mut pk_alt: [u8; PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_PUBLICKEYBYTES] = mem::uninitialized();
            let mut sk_alt: [u8; PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_SECRETKEYBYTES] = mem::uninitialized();
            let mut detached_sig: [u8; PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_BYTES] = mem::uninitialized();
            let mut sm = Vec::with_capacity(mlen + PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_BYTES);
            let mut smlen = 0;
            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr())
            );
            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign(
                    sm.as_mut_ptr(), &mut smlen as *mut usize,
                    msg.as_ptr(), mlen, sk.as_ptr())
            );
            sm.set_len(smlen);

            let mut unpacked_m = Vec::with_capacity(mlen + PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_BYTES);
            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_open(
                    unpacked_m.as_mut_ptr(), &mut mlen as *mut usize,
                    sm.as_ptr(), sm.len(),
                    pk.as_ptr()
                )
            );
            unpacked_m.set_len(mlen);
            assert_eq!(unpacked_m, msg);

            // check verification fails with wrong pk
            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_keypair(pk_alt.as_mut_ptr(), sk_alt.as_mut_ptr())
            );
            assert_eq!(
                -1,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_open(
                    unpacked_m.as_mut_ptr(), &mut mlen as *mut usize,
                    sm.as_ptr(), sm.len(),
                    pk_alt.as_ptr()
                )
            );

            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_signature(
                    detached_sig.as_mut_ptr(), &mut smlen as *mut usize,
                    msg.as_ptr(), msg.len(),
                    sk.as_ptr())
            );
            assert_eq!(smlen, PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_CRYPTO_BYTES);
            assert_eq!(
                0,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(), smlen,
                    msg.as_ptr(), msg.len(),
                    pk.as_ptr())
            );
            assert_eq!(
                -1,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(), smlen,
                    msg.as_ptr(), msg.len(),
                    pk_alt.as_ptr())
            );

            assert_eq!(
                -1,
                PQCLEAN_SPHINCSSHAKE256128FSIMPLE_CLEAN_crypto_sign_verify(
                    detached_sig.as_ptr(), smlen,
                    msg.as_ptr(), msg.len()-1,
                    pk.as_ptr())
            );
        }
    }
}
