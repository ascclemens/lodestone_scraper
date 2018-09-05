use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "not found")]
  NotFound,
  #[fail(display = "Lodestone responded with an unexpected code: {}", _0)]
  UnexpectedResponse(StatusCode),
  #[fail(display = "network error: {}", _0)]
  Net(reqwest::Error),
  #[fail(display = "url parse error: {}", _0)]
  Url(url::ParseError),
  #[fail(display = "lodestone parse error: {}", _0)]
  Parse(lodestone_parser::error::Error),
}
