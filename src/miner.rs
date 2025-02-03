use {
    diesel:: {ExpressionMethods, Insertable, Queryable, RunQueryDsl},
    diesel::query_dsl::methods::FilterDsl,
    diesel::result::Error,
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,

    super::schema::miners,

    crate::DBPooledConnection,
    crate::wallet::fetch_wallet_by_id
};

// --------------------- JSON Payload (REST)

#[derive(Debug, Deserialize, Serialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}


// --------------------- POST Request Body for new Miner

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMinerRequest {
    nickname: String,
}


// --------------------- DAO Object (DB Table Records)

#[derive(Queryable, Insertable)]
#[table_name = "miners"]
pub struct MinerDAO {
    pub id: String,
    pub address: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}