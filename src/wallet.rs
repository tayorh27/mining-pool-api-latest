use {
    diesel:: {ExpressionMethods, Insertable, Queryable, RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    super::schema::wallets,

    crate::DBPooledConnection,
    crate::miner::{Miner, MinerDAO}
};

// --------------------- JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,
    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

impl Wallet {
    pub fn to_wallet_dao(&self) -> WalletDAO {
        WalletDAO { 
            address: Uuid::parse_str(self.address.as_str()).unwrap(), 
            club_name: self.club_name.to_string() 
        }
    }
}

// --------------------- POST Request Body for new Miner

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}


// --------------------- DAO Object (DB Table Records)

#[derive(Queryable, Insertable)]
#[table_name = "wallets"]
pub struct WalletDAO {
    pub address: Uuid,
    pub club_name: String,
}