use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rand::rngs::OsRng;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::{
    RsaPrivateKey, RsaPublicKey,
    pkcs8::{EncodePrivateKey, EncodePublicKey},
};
use sha2::{Digest, Sha256};

fn gen_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

fn pow(nickname: &str, difficulty: usize) -> (u64, Vec<u8>) {
    let mut nonce = 0u64;
    loop {
        let data = format!("{}{}", nickname, nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hash = hasher.finalize();
        if hash[..difficulty].iter().all(|&b| b == 0) {
            return (nonce, hash.to_vec());
        }
        nonce += 1;
    }
}

fn sign(private_key: &RsaPrivateKey, data: &[u8]) -> Vec<u8> {
    private_key
        .sign(Pkcs1v15Sign::new_unprefixed(), data)
        .expect("Failed to sign")
}

fn verify(public_key: &RsaPublicKey, data: &[u8], signature: &[u8]) -> bool {
    public_key
        .verify(Pkcs1v15Sign::new_unprefixed(), data, signature)
        .is_ok()
}

fn main() {
    let (private_key, public_key) = gen_keypair();
    println!(
        "Public Key:\n{}",
        public_key.to_public_key_pem(Default::default()).unwrap()
    );
    println!(
        "Private Key:\n{:?}",
        private_key.to_pkcs8_pem(Default::default()).unwrap()
    );

    let nickname = "gumi";
    let difficulty = 2;
    let (nonce, hash) = pow(nickname, difficulty);
    println!("POW found! Nonce: {}", nonce);
    println!("Hash: {}", BASE64_STANDARD.encode(&hash));

    let signature = sign(&private_key, &hash);
    println!("Signature: {}", BASE64_STANDARD.encode(&signature));

    let is_valid = verify(&public_key, &hash, &signature);
    println!("Signature valid? {}", is_valid);
}
