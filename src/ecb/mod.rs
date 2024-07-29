use maud::*;
use Error::*;
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No such Clip #{0}")]
    NotFoundErro(usize),
    #[error("You can't access Clip #{0}")]
    UnauthClip(usize),
}

impl From<Error> for maud::Markup {
    fn from(e: Error) -> Markup {
        html! {
            h1 { "EasyClipBoard error" };
            h2 { (e) };
        }
    }
}

pub async fn try_get(code: usize) -> Result<String, Error> {
    Err(NotFoundErro(code))
}
