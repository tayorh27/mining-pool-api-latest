use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::miner::*,
    crate::util::*,
};

// List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {

    // Get all MinerDAO objects from db and convert to miner object

    let miners: Vec<Miner> = Vec![];
    ResponseType::Ok(miners).get_response()
}

// Get a Miner
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    let miner:Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found".to_string())
        ).get_response()
    }
}

//Create new Miner
#[post("/wallet/{id}/miners")]
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    let miner: Vec<Miner> = Vec![];
    ResponseType::Created(miner).get_response()
}