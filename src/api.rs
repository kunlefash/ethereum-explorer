use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use web3::types::{Block, BlockId, U256};
use web3::{transports, Web3};

#[get("/block/{block_number}")]
async fn get_block_info(
    web::Path((block_number,)): web::Path<(String,)>,
    web3: web::Data<Arc<Web3<transports::Http>>>,
) -> impl Responder {
    match block_number.parse::<U256>() {
        Ok(number) => {
            let web3_clone = web3.clone();
            let block_info = spawn_blocking(move || {
                let eth = web3_clone.eth();
                eth.block(BlockId::Number(number))
                    .map_err(|e| HttpResponse::InternalServerError().body(format!("Failed to fetch block {}: {:?}", number, e)))
            })
            .await;

            match block_info {
                Ok(Ok(Some(block))) => {
                    HttpResponse::Ok().body(format!(
                        "Block {}: Hash: {}, Timestamp: {}",
                        number,
                        block.hash.unwrap(),
                        block.timestamp.unwrap()
                    ))
                }
                Ok(Ok(None)) => HttpResponse::NotFound().body(format!("Block {} not found", number)),
                Ok(Err(e)) => e,
                Err(_) => HttpResponse::InternalServerError().body("Failed to fetch block info"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body(format!("Invalid block number: {}", block_number)),
    }
}
