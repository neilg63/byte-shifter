<!-- #[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/byte-shifter)
[![crates.io](https://img.shields.io/crates/v/byte-shifter.svg)](https://crates.io/crates/byte-shifter)
[![docs.rs](https://docs.rs/byte-shifter/badge.svg)](https://docs.rs/byte-shifter)
 -->
# ByteShifter: Fast Data Encryption

ByteShifter is a Rust library that provides efficient functions for encrypting and decrypting byte streams using a cyclic byte-shifting algorithm. It works by applying a *mask* (an array of unsigned 8-bit integers) to shift bytes in a source array. The same mask can then be used to decrypt the data. This approach produces indecipherable binary blobs, making it suitable for encrypting files or streams.

---

## Features

- **Random Key Generation**: Generate random byte sequences to use as encryption keys.
- **Hexadecimal Conversion**: Convert byte arrays to and from hexadecimal strings.
- **Byte Shifting**: Encrypt and decrypt byte streams using a mask.
- **Stream Support**: Process data in chunks for efficient file encryption and decryption.

---

## Functions

### `generate_code(size: u8) -> Vec<u8>`
Generates a random byte sequence of the specified size, which can be used as an encryption key.

```rust
let mask = generate_code(32);
// Generates a random sequence of 32 numbers between 0 and 255.
```

---

### `generate_long_random_u8(size: usize) -> Vec<u8>`
Generates a long random byte sequence, useful for testing or adding noise to data.

```rust
let long_stream = generate_long_random_u8(1_000_000);
// `long_stream` is a vector of 1 million unsigned 8-bit integers (u8).
```

---

### `to_hexdec_string(bytes: &[u8]) -> String`
Converts a byte array to a hexadecimal string, where each byte is represented as two characters.

```rust
let mask = generate_code(4);
let mask_as_string = to_hexdec_string(&mask);
// Converts the byte array `mask` into a hexadecimal string.
```

---

### `from_hexdec_string(text: &str) -> Vec<u8>`
Converts a hexadecimal string slice back into a vector of bytes.

```rust
let mask_as_string = "09d28a76";
let mask_as_u8s = from_hexdec_string(&mask_as_string);
// `mask_as_u8s` can now be used to encode or decode bytes.
```

---

### `shift_bytes(source: &[u8], mask: &[u8]) -> Vec<u8>`
Encrypts a byte array by cyclically shifting its bytes using a mask. This function is most efficient when used with chunks of data from a stream reader. The result can be written directly to a file.

```rust
use std::fs::File;
use std::io::{Read, Write};

let mut chunk_file = File::open("/path/to/local/file.mp4")?;
let shifted_bytes = shift_bytes(&chunk_data, &mask);
chunk_file.write_all(&shifted_bytes)?;
```

---

### `unshift_bytes(source: &[u8], mask: &[u8]) -> Vec<u8>`
Decrypts a masked byte array by reversing the byte-shifting process. This function can process chunks from a stream reader to recreate the original byte sequence.

```rust
use std::fs::File;
use std::io::{Read, Write};

let file = File::open("/path/to/local/file.masked.bin")?;
let mut output_file = File::create("/path/to/local/file.mp4")?;
let mut buffer = [0u8; 1024];

loop {
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        break;
    }
    let translated = unshift_bytes(&buffer[..bytes_read], &mask);
    output_file.write_all(&translated)?;
}
```

---

## Use Cases

- **File Encryption**: Securely encrypt files by processing them in chunks.
- **Data Obfuscation**: Generate indecipherable binary blobs for sensitive data.
- **Testing**: Create large random byte streams for performance testing or noise generation.

---

## Installation

Add the following to your `Cargo.toml` to include ByteShifter as a dependency:

```toml
[dependencies]
byte-shifter = "0.1.0"
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
