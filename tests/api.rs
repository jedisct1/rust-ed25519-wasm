#![cfg(target_arch = "wasm32")]

use ed25519_wasm::{KeyPair, Noise, PublicKey, Seed, Signature};

const RFC8032_SEED: [u8; 32] = [
    0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c, 0xc4,
    0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae, 0x7f, 0x60,
];

const RFC8032_PK: [u8; 32] = [
    0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64, 0x07, 0x3a,
    0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07, 0x51, 0x1a,
];

#[test]
fn derives_public_key_from_seed() {
    let kp = KeyPair::from_seed(Seed::new(RFC8032_SEED));
    assert_eq!(&*kp.pk, &RFC8032_PK);
    assert_eq!(kp.sk.public_key(), kp.pk);
    kp.validate().unwrap();
}

#[test]
fn signs_and_verifies() {
    let kp = KeyPair::from_seed(Seed::new(RFC8032_SEED));
    let msg = b"test message";
    let signature = kp.sk.sign(msg, Some(Noise::new([7u8; 16])));

    kp.pk.verify(msg, &signature).unwrap();
    assert!(kp.pk.verify(b"different", &signature).is_err());

    let mut modified = signature;
    modified[3] ^= 1;
    assert!(kp.pk.verify(msg, &modified).is_err());
}

#[test]
fn signing_without_noise_uses_wasi_randomness() {
    let kp = KeyPair::from_seed(Seed::new(RFC8032_SEED));
    let msg = b"wasi random_get";
    let sig1 = kp.sk.sign(msg, None);
    let sig2 = kp.sk.sign(msg, None);

    kp.pk.verify(msg, &sig1).unwrap();
    kp.pk.verify(msg, &sig2).unwrap();
    assert_ne!(sig1, sig2);
}

#[test]
fn parses_public_types_from_slices() {
    let kp = KeyPair::from_seed(Seed::new(RFC8032_SEED));
    let pk = PublicKey::from_slice(kp.pk.as_ref()).unwrap();
    let signature = kp.sk.sign([], Some(Noise::new([9u8; 16])));
    let parsed = Signature::from_slice(signature.as_ref()).unwrap();

    assert_eq!(pk, kp.pk);
    pk.verify([], &parsed).unwrap();
    assert!(PublicKey::from_slice(&[0u8; 31]).is_err());
    assert!(Signature::from_slice(&[0u8; 63]).is_err());
}
