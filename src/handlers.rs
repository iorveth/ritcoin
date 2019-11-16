use crate::pending_pool;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SerializedTX {
    tx: Vec<u8>,
}

pub fn submit_tx(serialized_tx_form: web::Json<SerializedTX>) -> HttpResponse {
    if pending_pool::accept_serialized_transaction(&serialized_tx_form.tx).is_ok() {
        HttpResponse::Ok().body(format!("tx successfully saved to mempool"))
    } else {
        HttpResponse::BadRequest().body(format!("invalid tx"))
    }
}
