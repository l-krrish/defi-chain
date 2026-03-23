use clap::Parser;
use rpc::Cli;
use blockchain::chain::Chain;
use wallet::keypair::KeyPair;
use network::Node;
fn main() {
    let cli = Cli::parse();



    // create a chain
    // mine 2 blocks
    // generate a keypair
    // print the address
    // create a node and add a peer if cli.peer is Some

    let mut chain = Chain::new(cli.difficulty);
    chain.add_block("block 1 data".to_string());
    chain.add_block("block 2 data".to_string());

    let mut keypair = KeyPair::generate();
    let mut node = Node::new("node-1".to_string());

    
    println!("Chain length: {}", chain.blocks.len());
    println!("Tip hash: {}", chain.tip().hash);
    println!("Generated address: {}", keypair.address);

    let peer = cli.peer.clone();
    if let Some(peer) = peer {
        node.add_peer(peer);
    }

    rpc::run(cli);
}