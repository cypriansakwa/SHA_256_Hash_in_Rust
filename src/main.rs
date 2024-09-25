extern crate sha2;
use sha2::{Sha256, Digest};

fn main() {
    let input = b"Hello, World!";
    
    // Create a SHA-256 object
    let mut hasher = Sha256::new();
    
    // Write input data
    hasher.update(input);
    
    // Read the hash digest (output)
    let result = hasher.finalize();
    
    // Print the result as a hexadecimal string
    println!("SHA-256: {:x}", result);
}
