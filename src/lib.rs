/// This function provides functions to generate random byte sequences
/// that may serve as encryption keys 
pub fn generate_code(size: u8) -> Vec<u8> {
    (0..size).into_iter().map(|_| rand::random::<u8>()).collect()
}

/// This function provides functions to generate random very long random byte sequences
/// for testing purposes or to add noise to the data
pub fn generate_long_random_u8(size: usize) -> Vec<u8> {
    (0..size).into_iter().map(|_| rand::random::<u8>()).collect()
}

/// This function converts a byte array to a hexadecimal string
pub fn to_hexdec_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// This function converts hexadecimal string slice to a vector of bytes
pub fn from_hexdec_string(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&text[i..i + 2], 16).ok())
        .collect()
}

fn offset_bytes(source: &[u8], mask: &[u8], reverse: bool) -> Vec<u8> {
  let mask_len = mask.len();
  source.iter().enumerate().map(|(b_index, &b_val)| {
      let mask_index = b_index % mask_len;
      if reverse {
          b_val.wrapping_sub(mask[mask_index])
      } else {
          b_val.wrapping_add(mask[mask_index])
      }
  }).collect()
}

/// This function shifts the bytes of a source array by a mask array
pub fn shift_bytes(source: &[u8], mask: &[u8]) -> Vec<u8> {
  offset_bytes(source, mask, false)
}

/// This function unshifts the bytes of a masked source array by a mask array
pub fn unshift_bytes(source: &[u8], mask: &[u8]) -> Vec<u8> {
  offset_bytes(source, mask, true)
}


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_random_numbers() {
    let sequence = generate_code(8);
    let hexdec_value = to_hexdec_string(&sequence);

    assert_eq!(hexdec_value.len(), 16);
  }

  #[test]
  fn test_random_sequence_conversion() {
    let sequence = generate_code(8);
    let hexdec_value = to_hexdec_string(&sequence);
    let bytes = from_hexdec_string(&hexdec_value);
    assert_eq!(sequence, bytes);
  }

  #[test]
  fn test_simple_shift() {
    let source_str = "1234567890abcdef";
    let mask_str = "0102030405";
    let source = source_str.as_bytes();
    let mask = from_hexdec_string(mask_str);
    let masked_bytes = shift_bytes(&source, &mask);
    let shifted_str = masked_bytes.iter().map(|&b| b as char).collect::<String>();
    let expected_str = "2468:79;=5bdfhjg";
    assert_eq!(shifted_str, expected_str);

    assert_eq!(source, unshift_bytes(&masked_bytes, &mask));
  }

  #[test]
  fn test_one_mega_byte() {
      let mb_1 = 1024 * 1024;
    let source = generate_long_random_u8(mb_1);
    let mask = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]; 
    
    let masked_bytes = shift_bytes(&source, &mask);
    
    assert_eq!(source.len(), masked_bytes.len());

    assert_eq!(source.len(), mb_1)
  }

  #[test]
  fn test_hex_string_conversion() {
    let byte_array = [8, 11, 7]; // low values
    let hexdec_value = to_hexdec_string(&byte_array);
    let bytes = from_hexdec_string(&hexdec_value);
    assert_eq!(byte_array, bytes.as_slice());
    // Check the length of the hex string
    // each byte is represented by 2 hex digits with zero padding
    assert_eq!(hexdec_value.len(), 6);
  }
}
