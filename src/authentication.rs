use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

pub struct Authentication {
    authority: String,
}

impl Authentication {
    pub async fn validate_token(&self, token: String) -> Result<Claims, String> {
        let jwks = self
            .get_jwks(&format!("https://{}/.well-known/jwks.json", self.authority))
            .await;

        let header = decode_header(&token).unwrap();
        let kid = header.kid.unwrap();
        let jwk = jwks.find(&kid).unwrap();

        if let Some(j) = jwks.find(&kid) {
            match j.algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                    let mut validation = Validation::new(j.common.algorithm.unwrap());
                    validation.validate_exp = false;
                    let token = decode::<Claims>(&token, &decoding_key, &validation).unwrap();
                    Ok(token.claims)
                }
                _ => unreachable!("this should be an RSA"),
            }
        } else {
            return Err("No matching JWK found for the given kid".into());
        }
    }

    async fn get_jwks(&self, uri: &str) -> jwk::JwkSet {
        reqwest::get(uri).await.unwrap().json().await.unwrap()
    }
}
