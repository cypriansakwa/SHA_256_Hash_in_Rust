## Overview
This project demonstrates how to compute the SHA-256 hash of a string in Rust using the sha2 crate. It provides a simple example of creating a SHA-256 object, updating it with input data, and obtaining the hash digest in a hexadecimal format.
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- Add the sha2 crate to your Cargo.toml file to include the SHA-256 functionality.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
## Output
- When you run the program, it will output the SHA-256 hash of the string "Hello, World!" in hexadecimal format:
>```
>SHA-256: dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f
## Modifying the Input
- You can change the input string by modifying the input variable. For example:
>```
>let input = b"Your new input here!";
## Code Explanation
- **Dependencies:** The `sha2` crate provides the `Sha256` struct for computing SHA-256 hashes, along with the `Digest` trait for creating and updating hash values.
- **Creating the Hasher:** We initialize a new SHA-256 object using `Sha256::new()`, which is the state of the hash computation.
- **Updating the Hasher:** The input string `"Hello, World!"` (as bytes) is provided to the `hasher.update()` function, which processes the input data.
- **Finalizing the Hash:** After processing the input, we call `hasher.finalize()` to complete the computation and produce the final hash value
- **Displaying the Hash:** The resulting hash is printed in hexadecimal format using the `println!` macro.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/SHA_256_Hash_in_Rust.git
   cd SHA_256_Hash_in_Rust
