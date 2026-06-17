//! Ed25519 signatures for WebAssembly backed by linked lib25519 code.

use core::convert::TryFrom;
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic;

mod ffi {
    extern "C" {
        pub fn crypto_sign(sm: *mut u8, smlen: *mut i64, m: *const u8, mlen: i64, sk: *const u8);
        pub fn crypto_sign_open(
            m: *mut u8,
            mlen: *mut i64,
            sm: *const u8,
            smlen: i64,
            pk: *const u8,
        ) -> i32;
        pub fn crypto_nG_merged25519(pk: *mut u8, sk: *const u8);
        pub fn crypto_hash_sha512(out: *mut u8, input: *const u8, len: i64);
        pub fn randombytes_seed_bytes(seed: *const u8, seedlen: i64);
    }
}

/// Errors returned by parsing and verification operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The signature does not verify.
    SignatureMismatch,
    /// The public key is invalid.
    InvalidPublicKey,
    /// The secret key is invalid.
    InvalidSecretKey,
    /// The signature is invalid.
    InvalidSignature,
    /// The seed does not have the expected length.
    InvalidSeed,
    /// The noise does not have the expected length.
    InvalidNoise,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SignatureMismatch => write!(f, "signature doesn't verify"),
            Error::InvalidPublicKey => write!(f, "invalid public key"),
            Error::InvalidSecretKey => write!(f, "invalid secret key"),
            Error::InvalidSignature => write!(f, "invalid signature"),
            Error::InvalidSeed => write!(f, "invalid seed length"),
            Error::InvalidNoise => write!(f, "invalid noise length"),
        }
    }
}

impl std::error::Error for Error {}

/// A seed, from which an Ed25519 key pair can be derived.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Seed([u8; Seed::BYTES]);

impl From<[u8; Seed::BYTES]> for Seed {
    fn from(seed: [u8; Seed::BYTES]) -> Self {
        Seed(seed)
    }
}

impl Seed {
    /// Number of raw bytes in a seed.
    pub const BYTES: usize = 32;

    /// Creates a seed from raw bytes.
    pub fn new(seed: [u8; Seed::BYTES]) -> Self {
        Seed(seed)
    }

    /// Creates a seed from a slice.
    pub fn from_slice(seed: &[u8]) -> Result<Self, Error> {
        let mut out = [0u8; Seed::BYTES];
        if seed.len() != out.len() {
            return Err(Error::InvalidSeed);
        }
        out.copy_from_slice(seed);
        Ok(Seed(out))
    }

    /// Generates a random seed.
    pub fn generate() -> Self {
        let mut seed = [0u8; Seed::BYTES];
        getrandom::fill(&mut seed).expect("RNG failure");
        Seed(seed)
    }

    /// Overwrites the seed in place.
    pub fn wipe_mut(&mut self) {
        wipe_bytes(&mut self.0);
    }
}

impl Default for Seed {
    fn default() -> Self {
        Seed::generate()
    }
}

impl Deref for Seed {
    type Target = [u8; Seed::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Seed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Per-signature noise for lib25519's randomized signing path.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Noise([u8; Noise::BYTES]);

impl Noise {
    /// Number of raw bytes in a noise value.
    pub const BYTES: usize = 16;

    /// Creates noise from raw bytes.
    pub fn new(noise: [u8; Noise::BYTES]) -> Self {
        Noise(noise)
    }

    /// Creates noise from a slice.
    pub fn from_slice(noise: &[u8]) -> Result<Self, Error> {
        let mut out = [0u8; Noise::BYTES];
        if noise.len() != out.len() {
            return Err(Error::InvalidNoise);
        }
        out.copy_from_slice(noise);
        Ok(Noise(out))
    }

    /// Generates random per-signature noise.
    pub fn generate() -> Self {
        let mut noise = [0u8; Noise::BYTES];
        getrandom::fill(&mut noise).expect("RNG failure");
        Noise(noise)
    }
}

impl Default for Noise {
    fn default() -> Self {
        Noise::generate()
    }
}

impl Deref for Noise {
    type Target = [u8; Noise::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Noise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Noise {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// An Ed25519 public key.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicKey([u8; PublicKey::BYTES]);

impl PublicKey {
    /// Number of raw bytes in a public key.
    pub const BYTES: usize = 32;

    /// Creates a public key from raw bytes.
    pub fn new(pk: [u8; PublicKey::BYTES]) -> Self {
        PublicKey(pk)
    }

    /// Creates a public key from a slice.
    pub fn from_slice(pk: &[u8]) -> Result<Self, Error> {
        let mut out = [0u8; PublicKey::BYTES];
        if pk.len() != out.len() {
            return Err(Error::InvalidPublicKey);
        }
        out.copy_from_slice(pk);
        Ok(PublicKey(out))
    }

    /// Verifies that `signature` is valid for `message`.
    pub fn verify(&self, message: impl AsRef<[u8]>, signature: &Signature) -> Result<(), Error> {
        verify(self, message.as_ref(), signature)
    }
}

impl Deref for PublicKey {
    type Target = [u8; PublicKey::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// An Ed25519 secret key: 32-byte seed followed by the 32-byte public key.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SecretKey([u8; SecretKey::BYTES]);

impl SecretKey {
    /// Number of raw bytes in a secret key.
    pub const BYTES: usize = Seed::BYTES + PublicKey::BYTES;

    /// Creates a secret key from raw bytes.
    pub fn new(sk: [u8; SecretKey::BYTES]) -> Self {
        SecretKey(sk)
    }

    /// Creates a secret key from a slice.
    pub fn from_slice(sk: &[u8]) -> Result<Self, Error> {
        let mut out = [0u8; SecretKey::BYTES];
        if sk.len() != out.len() {
            return Err(Error::InvalidSecretKey);
        }
        out.copy_from_slice(sk);
        Ok(SecretKey(out))
    }

    /// Returns the public counterpart of this secret key.
    pub fn public_key(&self) -> PublicKey {
        let mut pk = [0u8; PublicKey::BYTES];
        pk.copy_from_slice(&self.0[Seed::BYTES..]);
        PublicKey(pk)
    }

    /// Returns the seed portion of this secret key.
    pub fn seed(&self) -> Seed {
        let mut seed = [0u8; Seed::BYTES];
        seed.copy_from_slice(&self.0[..Seed::BYTES]);
        Seed(seed)
    }

    /// Recomputes the public key and checks that it matches `pk`.
    pub fn validate_public_key(&self, pk: &PublicKey) -> Result<(), Error> {
        let kp = KeyPair::from_seed(self.seed());
        if kp.pk == *pk {
            Ok(())
        } else {
            Err(Error::InvalidPublicKey)
        }
    }

    /// Computes a signature for `message`.
    ///
    /// If `noise` is `None`, lib25519 obtains fresh randomness from WASI.
    pub fn sign(&self, message: impl AsRef<[u8]>, noise: Option<Noise>) -> Signature {
        sign(self, message.as_ref(), noise)
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        wipe_bytes(&mut self.0);
    }
}

impl Deref for SecretKey {
    type Target = [u8; SecretKey::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SecretKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// An Ed25519 key pair.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct KeyPair {
    /// Public key part of the key pair.
    pub pk: PublicKey,
    /// Secret key part of the key pair.
    pub sk: SecretKey,
}

impl KeyPair {
    /// Number of bytes in a serialized key pair.
    pub const BYTES: usize = SecretKey::BYTES;

    /// Generates a new key pair from a random seed.
    pub fn generate() -> KeyPair {
        KeyPair::from_seed(Seed::generate())
    }

    /// Derives a key pair from a seed.
    pub fn from_seed(seed: Seed) -> KeyPair {
        assert!(
            seed.iter().fold(0, |acc, byte| acc | byte) != 0,
            "All-zero seed"
        );

        let mut az = [0u8; 64];
        unsafe {
            ffi::crypto_hash_sha512(az.as_mut_ptr(), seed.as_ptr(), Seed::BYTES as i64);
        }
        az[0] &= 248;
        az[31] &= 63;
        az[31] |= 64;

        let mut pk = [0u8; PublicKey::BYTES];
        unsafe {
            ffi::crypto_nG_merged25519(pk.as_mut_ptr(), az.as_ptr());
        }
        wipe_bytes(&mut az);

        let mut sk = [0u8; SecretKey::BYTES];
        sk[..Seed::BYTES].copy_from_slice(seed.as_ref());
        sk[Seed::BYTES..].copy_from_slice(&pk);
        KeyPair {
            pk: PublicKey(pk),
            sk: SecretKey(sk),
        }
    }

    /// Creates a key pair from serialized secret-key bytes.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let sk = SecretKey::from_slice(bytes)?;
        let pk = sk.public_key();
        Ok(KeyPair { pk, sk })
    }

    /// Recomputes the public key and checks that it matches the secret key.
    pub fn validate(&self) -> Result<(), Error> {
        self.sk.validate_public_key(&self.pk)
    }
}

impl Deref for KeyPair {
    type Target = [u8; KeyPair::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.sk
    }
}

impl DerefMut for KeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sk
    }
}

/// An Ed25519 signature.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Signature([u8; Signature::BYTES]);

impl Signature {
    /// Number of raw bytes in a signature.
    pub const BYTES: usize = 64;

    /// Creates a signature from raw bytes.
    pub fn new(signature: [u8; Signature::BYTES]) -> Self {
        Signature(signature)
    }

    /// Creates a signature from a slice.
    pub fn from_slice(signature: &[u8]) -> Result<Self, Error> {
        let mut out = [0u8; Signature::BYTES];
        if signature.len() != out.len() {
            return Err(Error::InvalidSignature);
        }
        out.copy_from_slice(signature);
        Ok(Signature(out))
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x?}", &self.0)
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Signature::from_slice(slice)
    }
}

impl Deref for Signature {
    type Target = [u8; Signature::BYTES];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

fn sign(sk: &SecretKey, message: &[u8], noise: Option<Noise>) -> Signature {
    let mlen = i64_len(message.len());
    let signed_len = Signature::BYTES
        .checked_add(message.len())
        .expect("message is too long");
    let mut signed = Vec::<u8>::with_capacity(signed_len);
    let mut actual_len = 0i64;

    unsafe {
        if let Some(noise) = noise.as_ref() {
            ffi::randombytes_seed_bytes(noise.as_ptr(), Noise::BYTES as i64);
        }
        ffi::crypto_sign(
            signed.as_mut_ptr(),
            &mut actual_len,
            message.as_ptr(),
            mlen,
            sk.as_ptr(),
        );
        assert_eq!(actual_len, i64_len(signed_len), "lib25519 signing failed");
        signed.set_len(signed_len);
    }

    let mut signature = [0u8; Signature::BYTES];
    signature.copy_from_slice(&signed[..Signature::BYTES]);
    Signature(signature)
}

fn verify(pk: &PublicKey, message: &[u8], signature: &Signature) -> Result<(), Error> {
    let signed_len = Signature::BYTES
        .checked_add(message.len())
        .expect("message is too long");
    let mut signed = Vec::<u8>::with_capacity(signed_len);
    signed.extend_from_slice(signature.as_ref());
    signed.extend_from_slice(message);

    let mut opened = Vec::<u8>::with_capacity(signed_len);
    let mut opened_len = 0i64;
    let ret = unsafe {
        ffi::crypto_sign_open(
            opened.as_mut_ptr(),
            &mut opened_len,
            signed.as_ptr(),
            i64_len(signed_len),
            pk.as_ptr(),
        )
    };
    if ret != 0 {
        return Err(Error::SignatureMismatch);
    }

    let opened_len = usize::try_from(opened_len).map_err(|_| Error::SignatureMismatch)?;
    if opened_len != message.len() {
        return Err(Error::SignatureMismatch);
    }
    unsafe {
        opened.set_len(opened_len);
    }
    if opened == message {
        Ok(())
    } else {
        Err(Error::SignatureMismatch)
    }
}

fn i64_len(len: usize) -> i64 {
    i64::try_from(len).expect("message is too long")
}

fn wipe_bytes(bytes: &mut [u8]) {
    for byte in bytes {
        unsafe {
            ptr::write_volatile(byte, 0);
        }
    }
    atomic::compiler_fence(atomic::Ordering::SeqCst);
    atomic::fence(atomic::Ordering::SeqCst);
}
