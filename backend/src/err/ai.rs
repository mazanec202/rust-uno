use crate::err::draw_cards::PlayerDrawError;
use crate::err::play_card::PlayCardError;
use crate::err::status::CreateStatusError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AiError {
    PlayCardError(PlayCardError),
    DrawCardError(PlayerDrawError),
    CreateStatusError(CreateStatusError),
}

impl Error for AiError {}

impl Display for AiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AiError::*;

        match self {
            PlayCardError(err) => write!(f, "{}", err),
            DrawCardError(err) => write!(f, "{}", err),
            CreateStatusError(err) => write!(f, "{}", err),
        }
    }
}

impl From<PlayCardError> for AiError {
    fn from(e: PlayCardError) -> Self {
        AiError::PlayCardError(e)
    }
}

impl From<PlayerDrawError> for AiError {
    fn from(e: PlayerDrawError) -> Self {
        AiError::DrawCardError(e)
    }
}

impl From<CreateStatusError> for AiError {
    fn from(e: CreateStatusError) -> Self {
        AiError::CreateStatusError(e)
    }
}
