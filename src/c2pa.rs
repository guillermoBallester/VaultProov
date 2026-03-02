pub fn parse_manifest(bytes: &[u8]) {
    let mut i = 8;
    while i + 8 <= bytes.len() {
        let some_box = JumboBox::from_bytes(&bytes[i..]);
        println!(
            "box size: {}, box type {:?}",
            some_box.size,  std::str::from_utf8(&some_box.box_type).unwrap()
        );
        i += some_box.size as usize;
    }
}

pub struct JumboBox {
    pub size: u32,
    pub box_type: [u8; 4],
}

impl JumboBox {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            size: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            box_type: bytes[4..8].try_into().unwrap(),
        }
    }
}
