use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
const SECRET: &str = "acli-secret";

pub fn process_jwt_encode(sub: String, aud: String, exp: u64) -> Result<String> {
    let claims = Claims::new(sub, aud, exp);
    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))?;
    Ok(token)
}

pub fn process_jwt_decode(token: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);

    // 要设置验证过期时间 但是不验证目标
    validation.validate_aud = false;
    validation.validate_exp = true;
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

impl Claims {
    pub fn new(sub: String, aud: String, exp: u64) -> Self {
        // normalize the timestamps by stripping of microseconds
        Self { sub, aud, exp }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhcmp1bkBhLmNvbSIsImF1ZCI6ImRldmljZTEiLCJleHAiOjEwMDAwMDAwMDAwfQ.aWltfQbbIF5sArVSZjNFPU-qeUS77E4b9AEahmeXlV8";

    use super::Claims;

    #[test]
    fn test_round_trip() -> Result<()> {
        let sub = "arjun@a.com".to_string();
        let aud = "device1".to_string();
        let exp = 10000000000;
        let claims = Claims::new(sub.clone(), aud.clone(), exp);

        let token = process_jwt_encode(sub.clone(), aud.clone(), exp)?;

        assert_eq!(&token, EXPECTED_TOKEN);

        let decoded_claims = process_jwt_decode(&token)?;

        assert_eq!(decoded_claims, claims);

        Ok(())
    }

    #[test]
    fn test_jwt_sign() -> Result<()> {
        let sub = "arjun@a.com".to_string();
        let aud = "device1".to_string();
        let exp = 10000000000;

        let token = process_jwt_encode(sub.clone(), aud.clone(), exp)?;

        assert_eq!(&token, EXPECTED_TOKEN);
        Ok(())
    }

    #[test]
    fn test_jwt_verify() -> Result<()> {
        let sub = "arjun@a.com".to_string();
        let aud = "device1".to_string();
        let exp = 10000000000;
        let claims = Claims::new(sub.clone(), aud.clone(), exp);

        let decoded_claims = process_jwt_decode(EXPECTED_TOKEN)?;

        assert_eq!(decoded_claims, claims);

        Ok(())
    }

    #[test]
    fn should_fail_on_invalid_timestamp() {
        // A token with the expiry of i64::MAX + 1
        let decode_result = process_jwt_decode(EXPECTED_TOKEN);
        assert!(decode_result.is_err());
    }
}
