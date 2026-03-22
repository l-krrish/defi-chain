pub struct Node {
    peer_id: String, 
    known_peers: Vec<String>,
}

impl Node {
    pub fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            known_peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, addr: String) {
        if !self.known_peers.contains(&addr) {
            self.known_peers.push(addr);
        }
    }

    pub fn broadcast(&self, message: String) {
        for peer in &self.known_peers {
            println!("Sending message to {}: {}", peer, message);
        }
    }

    pub fn receive(&self, from: &str, message: &str) {
    println!("[receive] from {}: {}", from, message);
    // re-broadcast to all peers except sender
    for peer in &self.known_peers {
        if peer != from {
            println!("[gossip] -> {} : {}", peer, message);
        }
    }
}

}