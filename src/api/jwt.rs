use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Error;
use sqlx::types::Uuid;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaim {
    session_id: String,
}

#[derive(Clone)]
pub struct JwtKeys {
    pub(crate) encode_key: EncodingKey,
    pub(crate) decode_key: DecodingKey,
}
impl JwtClaim {
    pub fn new(session_id: Uuid) -> Self {
        let session_id = session_id.to_string();

        Self { session_id }
    }
}

impl JwtClaim {
    pub fn try_into_uuid(self) -> Result<Uuid, Error> {
        match Uuid::from_str(&self.session_id) {
            Ok(uuid) => Ok(uuid),
            Err(err) => Err(err),
        }
    }
}

/// Generates a JWT validation configuration object.
///
/// This function creates a `Validation` instance specifically configured
/// for validating JWTs signed using the HMAC SHA-256 algorithm (`HS256`).
/// It modifies the default `Validation` object by making these adjustments:
///
/// - Clears the set of required claims (`required_spec_claims`) to allow more flexibility.
/// - Disables audience (`aud`) claim validation by setting `validate_aud` to `false`.
///
/// # Returns
///
/// A `Validation` object configured with the above settings.
///
/// # Example
///
/// ```
/// use jsonwebtoken::{Algorithm, Validation};
///
/// let validation = get_jwt_validation();
///
/// assert_eq!(validation.algorithms, vec![Algorithm::HS256]);
/// assert!(validation.required_spec_claims.is_empty());
/// assert!(!validation.validate_aud);
/// ```
///
/// This can be used to validate incoming JWT tokens against the specific setup.
///
/// # Note
///
/// - Make sure the returned `Validation` instance matches the requirements
///   of your application's JWT validation logic.
/// - Disabling `validate_aud` means that audience verification will not occur;
///   this could reduce security if the audience claim is required by your use case.
///
/// # Dependencies
///
/// This function assumes you have imported:
/// - `jsonwebtoken::{Algorithm, Validation}`
/// - `std::collections::HashSet`
///
/// # See Also
///
/// For more details, refer to the `jsonwebtoken` crate documentation:
/// https://docs.rs/jsonwebtoken/latest/jsonwebtoken/
pub fn get_jwt_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.required_spec_claims = HashSet::new();
    validation.validate_aud = false;

    validation
}

/// Generates JWT (JSON Web Token) keys for encoding and decoding using the provided secret.
///
/// # Parameters
/// - `secret`: A `Vec<u8>` containing the secret key used to create
///    both the encoding and decoding keys.
///
/// # Returns
/// - `JwtKeys`: A structure containing:
///     - `encode_key`: An `EncodingKey` generated from the provided secret,
///       to be used for creating JWTs.
///     - `decode_key`: A `DecodingKey` generated from the same secret,
///       to be used for verifying and decoding JWTs.
///
/// # Example
/// ```
/// let secret = b"my_secret_key".to_vec();
/// let jwt_keys = get_jwt_keys(secret);
/// ```
///
/// # Notes
/// - The provided `secret` should be a secure and random sequence of bytes
///   to ensure the safety of the JWT tokens.
/// - This function returns a `JwtKeys` struct that needs to implement
///   to_owned for creating the owned instance of the keys.
///
/// # Dependencies
/// Ensure the usage of the `jsonwebtoken` crate for accessing `EncodingKey`
/// and `DecodingKey`.
pub fn get_jwt_keys(secret: Vec<u8>) -> JwtKeys {
    JwtKeys {
        encode_key: EncodingKey::from_secret(secret.as_ref()),
        decode_key: DecodingKey::from_secret(secret.as_ref()),
    }
    .to_owned()
}
