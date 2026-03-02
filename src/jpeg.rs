pub fn is_jpeg(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
       false
    } else {
        bytes[0] == 0xFF && bytes[1] == 0xD8
            && bytes[bytes.len() - 1] == 0xD9 && bytes[bytes.len() - 2] == 0xFF
    }
}

pub fn find_c2pa_signature(bytes: &[u8]) -> Option<(usize, usize)> {
    let mut i = 2;
    while i + 3 < bytes.len() {
        let length = (bytes[i+2] as usize) * 256 + bytes[i+3] as usize;
        println!("Marker: 0xFF 0x{:02X} at position {}, length {}", bytes[i+1], i, length);
        if bytes[i] == 0xFF && bytes[i+1] == 0xEB {
            return Some((i, length));
        }
        i = i + 2 + length
    }
    None
}