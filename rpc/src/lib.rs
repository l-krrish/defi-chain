use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetBlock { index: u64 },
    SendTransaction { from: String, to: String, amount: u64 },
    Swap { amount_a: u64 },
    Borrow { owner: String, amount: u64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Block { index: u64, hash: String },
    TransactionSent { success: bool },
    SwapResult { output: u64 },
    BorrowResult { success: bool },
    Error { message: String },
}

pub fn handle(request: Request) -> Response {
    match request {
        Request::GetBlock { index } => Response::Block { index, hash: String::from("dummy_hash") },
        // fill in the rest...
        Request::SendTransaction { from, to, amount } => Response::TransactionSent { success: true },
        Request::Swap { amount_a } => Response::SwapResult { output: amount_a /2 },
        Request::Borrow { owner, amount } => Response::BorrowResult { success: true },
    }
}