use crate::pending_pool;
use crate::serializer;
use crate::*;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
//ritcoin_state: web::Data<Arc<RitCoinState>>
#[derive(Deserialize)]
pub struct SerializedTX {
    tx: Vec<u8>,
}

pub fn handle_submit_tx(serialized_tx_form: web::Json<SerializedTX>) -> HttpResponse {
    if pending_pool::accept_serialized_transaction(&serialized_tx_form.tx).is_ok() {
        HttpResponse::Ok().body(format!("tx successfully saved to mempool"))
    } else {
        HttpResponse::BadRequest().body(format!("invalid tx"))
    }
}

pub fn handle_pendings() -> Result<HttpResponse, HttpResponse> {
    let pending_transactions = pending_pool::get_last_transactions_deserialized(None);
    pending_transactions
        .map(|transactions| HttpResponse::Ok().json(transactions))
        .map_err(|e| HttpResponse::InternalServerError().body(format!("{:?}", e)))
}
