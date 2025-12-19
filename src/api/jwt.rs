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

pub fn get_jwt_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.required_spec_claims = HashSet::new();
    validation.validate_aud = false;

    validation
}

pub fn get_jwt_keys(secret: Vec<u8>) -> JwtKeys {
    JwtKeys {
        encode_key: EncodingKey::from_secret(secret.as_ref()),
        decode_key: DecodingKey::from_secret(secret.as_ref()),
    }
    .to_owned()
}
