use jsonwebtoken as jwt;
pub use jsonwebtoken::errors::Error;

const PRIV_PEM: &str = include_str!("../.priv.pem");

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims<T>
where
    T: serde::Serialize,
{
    exp: u64,
    info: T,
}

pub fn sign<T>(info: T) -> Result<String, Error>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let claim = Claims::<T> {
        exp: 10000000000,
        info,
    };
    jwt::encode(
        &jwt::Header::default(),
        &claim,
        &jwt::EncodingKey::from_secret(PRIV_PEM.as_bytes()),
    )
}

pub fn verify<T>(jwt: &str) -> Result<T, Error>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    jwt::decode::<Claims<T>>(
        jwt,
        &jwt::DecodingKey::from_secret(PRIV_PEM.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS256),
    )
    .map(move |a| a.claims.info)
}
