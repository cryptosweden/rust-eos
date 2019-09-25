use crate::constant::*;
use crate::error;
use crate::public::PublicKey;
use crate::secret::SecretKey;
use crate::signature::Signature;
use rand::Rng;


/// A secp256k1 keypair.
#[derive(Clone, PartialEq, Eq)]
pub struct Keypair {
    /// The secret half of this keypair.
    pub sk: SecretKey,
    /// The public half of this keypair.
    pub pk: PublicKey,
}

impl Keypair {
    /// Generate an secp256k1 keypair.
    pub fn generate<R>(csprng: &mut R) -> Keypair where R:  Rng {
        let sk = SecretKey::generate(csprng);
        let pk = PublicKey::from(&sk);

        Keypair { sk, pk }
    }

    /// Generate an secp256k1 keypair from secret in WIF format
    pub fn from_secret_wif(wif: &str) -> Result<Keypair, error::Error> {
        let sk = SecretKey::from_wif(wif)?;
        let pk = PublicKey::from(&sk);

        Ok(Keypair { sk, pk })
    }

    /// Convert this keypair to bytes.
    pub fn to_bytes(&self) -> [u8; KEYPAIR_LENGTH] {
        let mut bytes: [u8; KEYPAIR_LENGTH] = [0u8; KEYPAIR_LENGTH];

        bytes[..SECRET_KEY_SIZE].copy_from_slice(self.sk.to_bytes().as_slice());
        bytes[SECRET_KEY_SIZE..].copy_from_slice(self.pk.to_bytes().as_slice());

        bytes
    }

    /// Sign a message with this keypair's secret key.
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.sk.sign(&message)
    }

    /// Verify a signature on a message with this keypair's public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.pk.verify(&message, &signature)
    }
}

#[cfg(test)]
mod tests {
    use super::{Keypair, PublicKey};
    use alloc::string::ToString;
    use rand::thread_rng;

    #[test]
    fn keypair_generate_should_work() {
        let mut rng = thread_rng();
        let keypair = Keypair::generate(&mut rng);

        assert_eq!(PublicKey::from(&keypair.sk), keypair.pk);
    }

    #[test]
    fn keypair_from_secret_wif_should_work() {
        let wif = "5HrBLKfeEdqH9KLMv1daHLVjrXV3DGVERAkN5cdSSc58bzqqfT4";
        let keypair = Keypair::from_secret_wif(wif).unwrap();
        assert_eq!(keypair.pk.to_string(), "EOS8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm");
    }

    #[test]
    fn keypair_sign_should_work() {
        let wif = "5HrBLKfeEdqH9KLMv1daHLVjrXV3DGVERAkN5cdSSc58bzqqfT4";
        let keypair = Keypair::from_secret_wif(wif).unwrap();
        let message = "hello".as_bytes();
        let sig = keypair.sign(&message);
        assert_eq!(sig.to_string(),
                   "SIG_K1_JwWq5syfD1tBvELTB6bQwmWeeRepC5P86kzmv4Kx1vLbFahK4qAp74y7QtG7GYjfH8MdPoyWTP7ygVE6QoEFtXQrzhanLa");
    }

    #[test]
    fn keypair_verify_should_work() {
        let wif = "5HrBLKfeEdqH9KLMv1daHLVjrXV3DGVERAkN5cdSSc58bzqqfT4";
        let keypair = Keypair::from_secret_wif(wif).unwrap();
        let message = "hello".as_bytes();
        let sig = keypair.sign(&message);

        let verify = keypair.verify(&message, &sig);
        assert_eq!(verify, true);
    }
}
