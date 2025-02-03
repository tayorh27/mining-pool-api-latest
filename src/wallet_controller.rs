use {
    actix_web::HttpResponse,
    actix_web::web::Json,

    crate::wallet::*,
    crate::util::*,
};

// List all Wallets
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {

    // Get all WalletDAO objects from db and convert to wallet object

    let wallets: Vec<Wallet> = Vec![];
    ResponseType::Ok(wallets).get_response()
}

// Get a Wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    let wallet:Option<Wallet> = None;
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet/Club not found".to_string())
        ).get_response()
    }
}

//Create new Wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<NewWalletRequest>) -> HttpResponse {
    let wallet: Vec<Miner> = Vec![];
    ResponseType::Created(wallet).get_response()
}