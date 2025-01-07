use argon2::{Algorithm, Argon2, Params, Version};
use password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHashString, PasswordHasher, PasswordVerifier,
    SaltString,
};

use crate::{Error, Result};

#[derive(Debug, Clone, Default)]
pub struct Hasher {}

impl Hasher {
    fn argon2_instance<'a>() -> Argon2<'a> {
        let algorithm = Algorithm::default();
        let version = Version::default();
        let params = Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            None,
        )
        .expect("Failed to create Argon2 params");
        Argon2::new(algorithm, version, params)
    }

    /// Hash password to PHC string ($argon2id$v=19$...)
    pub(crate) fn argon2_hash(&self, value: &str) -> Result<PasswordHashString> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Self::argon2_instance();

        let password_hash = argon2
            .hash_password(value.as_bytes(), &salt)
            .map_err(|e| Error::FailedPasswordHashing(e.to_string()))?;
        Ok(password_hash.serialize())
    }

    /// Verify password against PHC string.
    pub(crate) fn verify(&self, value: &str, phc_hash: &str) -> Result<()> {
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in
        // the `Argon2` instance.
        let parsed_hash = PasswordHash::new(phc_hash)
            .map_err(|e| Error::FailedPasswordValidation(e.to_string()))?;
        Self::argon2_instance()
            .verify_password(value.as_bytes(), &parsed_hash)
            .map_err(|e| Error::FailedPasswordValidation(e.to_string()))
    }
}
