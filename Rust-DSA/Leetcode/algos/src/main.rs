// Import necessary libraries
use sui_sdk::crypto::Keypair;
use sui_sdk::crypto::ed25519::Ed25519Keypair;
use blake2::digest::{Digest, BLAKE2b};
use bs58::encode;
fn main() {
// Generate a keypair using Ed25519 algorithm
let keypair: Ed25519Keypair = Ed25519Keypair::generate();
let public_key = keypair.get_public_key();

// Concatenate signature scheme flag (0x00 for Ed25519) with public key bytes
let mut data: Vec<u8> = Vec::with_capacity(1 + public_key.as_ref().len());
data.push(0x00); // Signature scheme flag for Ed25519
data.extend_from_slice(public_key.as_ref());

// Hash the data using BLAKE2b
let mut hasher = BLAKE2b::new();
hasher.update(&data);
let hash = hasher.finalize();

// Encode the hash into base58 string
let address = encode(&hash);

println!("Address: {}", address);"
  
}