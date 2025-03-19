#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from]reqwest::Error),
    #[error("Incorrect data for `{0}` request")]
    IncorrectData(String),
    #[error("Some uncategorized error: `{0}`")]
    Other(String)
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::ser::Serializer,
    {
      serializer.serialize_str(self.to_string().as_ref())
    }
}