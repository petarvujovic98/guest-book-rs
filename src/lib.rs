/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://docs.near.org/develop/contracts/anatomy
 *
 */
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::Vector,
    env, log, near_bindgen,
    serde::Serialize,
};

// If the user attaches more than 0.01N the message is premium
const PREMIUM_PRICE: u128 = 10_u128.pow(22);

// Create a struct PostedMessage to keep track of important information
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq, Serialize)]
pub struct PostedMessage {
    premium: bool,
    sender: String,
    text: String,
}

impl PostedMessage {
    // Create a PostedMessage based on the text that was provided, the attached deposit, and the signer of the transaction.
    pub fn new(text: String) -> Self {
        Self {
            premium: env::attached_deposit() >= PREMIUM_PRICE,
            sender: env::signer_account_id().to_string(),
            text,
        }
    }
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    messages: Vector<PostedMessage>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Adds a new message under the name of the sender's account id.
    #[payable]
    pub fn add_message(&mut self, message: String) -> PostedMessage {
        let posted_message = PostedMessage::new(message);
        log!("{:?}", posted_message);
        self.messages.push(&posted_message);

        posted_message
    }

    // Returns an array of last N messages. Paginates the messages using the from_index and limit parameters.
    pub fn get_messages(&self, from_index: usize, limit: usize) -> Vec<PostedMessage> {
        // Collect the requested messages
        self.messages.iter().skip(from_index).take(limit).collect()
    }
}
