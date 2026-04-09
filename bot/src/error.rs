use serenity::prelude::SerenityError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error(transparent)]
    Serenity(#[from] Box<SerenityError>),
    #[error(transparent)]
    Val(#[from] std::env::VarError)
}