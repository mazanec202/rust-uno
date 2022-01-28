use std::error::Error;

use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct TypedErrMsg {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_of_error: String,
    message: String,
}

impl TypedErrMsg {

    pub fn new(type_of_error: &str, error: impl Error) -> Self {
        Self {
            type_of_error: type_of_error.into(),
            message: error.to_string(),
        }
    }

    pub fn new_from_scratch(type_of_error: &str, message: String) -> Self {
        Self {
            type_of_error: type_of_error.into(),
            message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ErrMsg {
    msg: String,
}

impl ErrMsg {
    pub fn new(message: &str) -> Self {
        Self {
            msg: message.into(),
        }
    }

    pub fn from(err: impl Error) -> Self {
        Self {
            msg: err.to_string(),
        }
    }
}

pub struct ErrResp {}
impl ErrResp {
    pub fn game_not_found(id: String) -> HttpResponse {
        HttpResponse::NotFound().json(ErrMsg {
            msg: format!("Game with id '{}' not found", id),
        })
    }

    pub fn jwt_game_id_does_not_match() -> HttpResponse {
        HttpResponse::Forbidden().json(ErrMsg::new(
            "Game id in the url does not match the one in JWT",
        ))
    }

    pub fn game_has_no_current_player() -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrMsg::new("Current player not found"))
    }
}
