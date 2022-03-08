use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};

pub async fn validate_token(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let authority = std::env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");
    let jwks_reply = fetch_jwks(&format!(
        "https://{}/.well-known/jwks.json",
        authority.as_str(),
    ))
    .await
    .unwrap();

    let jwks: jwk::JwkSet = serde_json::from_str(&jwks_reply).unwrap();

    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => return Err("Token doesn't have a `kid` header field".into()),
    };
    if let Some(j) = jwks.find(&kid) {
        match j.algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.validate_exp = false;
                let decoded_token =
                    decode::<serde_json::Value>(token, &decoding_key, &validation).unwrap();
                println!("{:?}", decoded_token);
            }
            _ => unreachable!("this should be an RSA"),
        }
    } else {
        return Err("No matching JWK found for the given kid".into());
    }

    Ok(true)
}

async fn fetch_jwks(uri: &str) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let res = reqwest::get(uri).await.unwrap().text().await.unwrap();
    return Ok(res);
}
