use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct Commitment {
    name: String,
    amount: u8,
}

trait commitment_scheme {
    fn commit<T: Display>(u: T) -> Result<Vec<Vec<u8>>, Box<dyn Error>>;
    fn open<T: Display, Hash: Display>(input: T, commitment: Hash) -> Result<bool, Box<dyn Error>>;
}

impl Commitment {
    fn new(name: String, amount: u8) -> Self {
        Self { name, amount }
    }
}

impl Display for Commitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, amount: {}
         ",
            self.name, self.amount
        )
    }
}

impl commitment_scheme for Commitment {
    fn commit<T: Display>(u: T) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let hash = sha256::digest(u.to_string());
        println!("hash: {:?}\n", hash);
        Ok(vec![hash.to_string().into_bytes()])
    }
    fn open<T: Display, Hash: Display>(input: T, commitment: Hash) -> Result<bool, Box<dyn Error>> {
        let hash_input = sha256::digest(input.to_string());
        println!("input: {}, commitment: {}", input, commitment);
        Ok(hash_input.to_string() == commitment.to_string())
    }
}

fn main() {
    let alex_auction = Commitment::new("Alice".to_string(), 10);
    println!("alex_auction details: {}", alex_auction);
    let new_aution = Commitment::commit(alex_auction.amount);
    // convert bytes to string
    let new_aution = &new_aution
        .unwrap()
        .iter()
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .join(", ");
    println!("new_aution: {}", new_aution);
    let confirm = Commitment::open(alex_auction.amount, &new_aution);
    println!("confirmation: {:?}", confirm);
}
