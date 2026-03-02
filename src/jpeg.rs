pub fn is_jpeg(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        bytes[0] == 0xFF && bytes[1] == 0xD8
            && bytes[bytes.len() - 1] == 0xD9 && bytes[bytes.len() - 2] == 0xFF
    } else {
        false
    }
}