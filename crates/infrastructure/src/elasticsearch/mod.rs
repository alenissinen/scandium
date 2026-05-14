use elasticsearch::{Elasticsearch, http::transport::Transport};

pub mod listing_index;

pub fn create_client(url: &str) -> Elasticsearch {
    let transport = Transport::single_node(url).expect("Failed to create Elasticsearch transport");

    Elasticsearch::new(transport)
}
