pub struct ByteWriter {
    pub buffer: Vec<u8>,
    pub offset: usize,
    pub bitoffset: u8,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            offset: 0,
            bitoffset: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.buffer.len()
    }

    pub fn get_raw(&self) -> &[u8] {
        &self.buffer
    }

    pub fn write_int(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self.offset += 4;
        self.bitoffset = 0;
    }

    pub fn write_int_endian(&mut self, data: i32, length: usize) {
        let bytes = data.to_le_bytes();

        self.buffer.extend_from_slice(&bytes[..length]);
        self.offset += length;
        self.bitoffset = 0;
    }

    pub fn write_short_endian(&mut self, data: i16) {
        self.buffer.extend_from_slice(&data.to_le_bytes());
        self.offset += 2;
        self.bitoffset = 0;
    }

    pub fn write_byte(&mut self, value: u8) {
        self.buffer.push(value);
        self.offset += 1;
        self.bitoffset = 0;
    }

    pub fn write_short(&mut self, value: i16) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
        self.offset += 2;
        self.bitoffset = 0;
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        let length = value.len() as i32;

        self.write_int(length);
        self.buffer.extend_from_slice(value);
        self.offset += value.len();
        self.bitoffset = 0;
    }

    pub fn write_string(&mut self, value: Option<&str>) {
        self.bitoffset = 0;

        match value {
            Some(s) => {
                let str_bytes = s.as_bytes();
                let str_length = str_bytes.len() as i32;

                if str_length < 900_001 {
                    self.write_int(str_length);
                    self.buffer.extend_from_slice(str_bytes);
                    self.offset += str_bytes.len();
                } else {
                    self.write_int(-1);
                }
            }

            None => {
                self.write_int(-1);
            }
        }
    }

    pub fn write_boolean(&mut self, value: bool) {
        self.bitoffset = 0;
        self.write_byte(if value { 1 } else { 0 });
    }

    pub fn write_vint(&mut self, mut data: i32) {
        self.bitoffset = 0;
        let mut final_bytes = Vec::new();

        loop {
            let mut byte = (data & 0x7F) as u8;
            data >>= 7;

            if data != 0 {
                byte |= 0x80;
            }

            final_bytes.push(byte);

            if data == 0 {
                break;
            }
        }

        self.buffer.extend(final_bytes.iter());
        self.offset += final_bytes.len();
    }

    pub fn write_long_long(&mut self, value: i64) {
        let high = (value >> 32) as i32;
        let low = (value & 0xFFFFFFFF) as i32;

        self.write_int(high);
        self.write_int(low);
    }
}
