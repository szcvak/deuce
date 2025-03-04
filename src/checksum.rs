use crate::math::ror32;
use log::error;

pub struct ChecksumEncoder {
    pub checksum: u32,
    pub checksum2: u32,
    pub enabled: bool,
}

impl ChecksumEncoder {
    pub fn new() -> Self {
        Self {
            checksum: 0,
            checksum2: 0,
            enabled: true,
        }
    }

    /// Resets the checksum.
    pub fn reset(&mut self) {
        self.checksum = 0;
    }

    /// Writes a boolean value into the checksum.
    pub fn write_boolean(&mut self, value: bool) {
        let int_val = if value { 13 } else { 7 };
        self.checksum = int_val + ror32(self.checksum, 31);
    }

    /// Writes a byte value into the checksum.
    pub fn write_byte(&mut self, value: u8) {
        self.checksum = ror32(self.checksum, 31) + (value as u32) + 11;
    }

    /// Writes a 32-bit integer into the checksum.
    pub fn write_int(&mut self, value: i32) {
        self.checksum = ror32(self.checksum, 31) + (value as u32) + 9;
    }

    /// Writes a variable-length int into the checksum.
    pub fn write_vint(&mut self, value: i32) {
        self.checksum = (value as u32) + ror32(self.checksum, 31) + 33;
    }

    /// Writes a variable-length long (split into high and low parts) into the checksum.
    ///
    /// This directly translates the Python logic:
    ///   self.checksum = low + ROR4(high + ROR4(self.checksum, 31) + 65, 31) + 88
    pub fn write_vlong(&mut self, high: i32, low: i32) {
        self.checksum = low as u32
            + ( (high as u32 + ror32(self.checksum, 31) + 65).rotate_right(31) )
            + 88;
    }

    // ... Additional methods (write_int8, write_int16, etc.) can be added similarly.
}

