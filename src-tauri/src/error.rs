use rust_xlsxwriter::XlsxError;
use strum::ParseError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from]reqwest::Error),
    #[error(transparent)]
    Parse(#[from]ParseError),
    #[error(transparent)]
    Xlsx(#[from]XlsxError),
    #[error("Incorrect data for `{0}` request")]
    IncorrectData(String),
    #[error("Some uncategorized error: `{0}`")]
    Other(String),
    #[error("No `{field:?}` field found for game `{game_id:?}`")]
    NoGameField {
      field: String,
      game_id: Uuid
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::ser::Serializer,
    {
      serializer.serialize_str(self.to_string().as_ref())
    }
}