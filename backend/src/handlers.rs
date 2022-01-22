use crate::jwt_generate::generate_jwt;
use crate::repo::game_repo::GameRepo;
use crate::repo::address_repo::AddressRepo;
use crate::InMemoryGameRepo;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct GamePostData {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameCreateResponse {
    gameID: String,
    server: String,
    token: String,
}

#[post("/GameCreateData")]
pub async fn create_game(
    data: web::Data<Arc<Mutex<InMemoryGameRepo>>>,
    address_repo: web::Data<Arc<AddressRepo>>,
    body: web::Json<GamePostData>,
) -> impl Responder {
    if body.name.is_empty() {
        return HttpResponse::BadRequest().json("Name of the player cannot be empty");
    }
    let game_result = data.lock().unwrap().create_game(body.name.clone()).await;

    HttpResponse::Created().json(GameCreateResponse {
        gameID: game_result.as_ref().unwrap().id.clone(),
        server: address_repo.full_local_address(),
        token: generate_jwt(body.name.clone(), game_result.as_ref().unwrap().id.clone()),
    })
}
