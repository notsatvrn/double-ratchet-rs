use x25519_dalek::SharedSecret;
use hkdf::Hkdf;
use ring_compat::digest::Sha512;
use core::convert::TryInto;

#[cfg(test)]
use crate::dh::gen_shared_secret;

pub fn kdf_rk(rk: &[u8; 32], dh_out: &SharedSecret) -> ([u8; 32], [u8; 32]) {
    let h = Hkdf::<Sha512>::new(Some(rk), dh_out.as_bytes());
    let mut okm = [0u8; 64];
    let info = b"Root Key Info";
    h.expand(info, &mut okm).unwrap();
    let (a, b) = okm.split_at(32);
    (a.try_into()
         .expect("Incorrect length"),
     b.try_into()
         .expect("Incorrect length"))
}

#[cfg(test)]
pub fn gen_ck() -> [u8; 32] {
    let shared_secret = gen_shared_secret();
    let rk = [0; 32];
    let (_, ck) = kdf_rk(&rk, &shared_secret);
    ck
}

#[cfg(test)]
mod tests {
    use crate::dh::gen_shared_secret;
    use crate::kdf_root::kdf_rk;

    #[test]
    fn kdf_root_ratchet() {
        let rk = [0; 32];
        let shared_secret = gen_shared_secret();
        let (rk1, _) = kdf_rk(&rk, &shared_secret);
        let (rk2, _) = kdf_rk(&rk1, &shared_secret);
        assert_ne!(rk1, rk2)
    }
}