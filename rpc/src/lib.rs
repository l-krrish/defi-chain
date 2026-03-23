use serde::{Deserialize, Serialize};
use clap::Parser;
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

#[derive(Parser, Debug)]
#[command(name = "defi-chain")]
#[command(about = "A DeFi blockchain node")]
pub struct Cli {
    #[arg(long, default_value = "3000")]
    pub port: u16,

    #[arg(long, default_value = "2")]
    pub difficulty: usize,

    #[arg(long)]
    pub peer: Option<String>,
}

pub fn run(cli: Cli) {
    println!("Starting node on port {}", cli.port);
    println!("Mining difficulty: {}", cli.difficulty);
    if let Some(peer) = cli.peer {
        println!("Connecting to peer: {}", peer);
    }
}