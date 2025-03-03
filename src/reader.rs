use bytes::{Buf, Bytes};
use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecodeError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    UnexpectedEof,
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> Self {
        DecodeError::IoError(err)
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(err: FromUtf8Error) -> Self {
        DecodeError::Utf8Error(err)
    }
}

pub struct ByteReader {
    buf: Bytes,
}

impl ByteReader {
    pub fn from(data: Bytes) -> Self {
        Self { buf: data }
    }

    pub fn remaining(&self) -> usize {
        self.buf.remaining()
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        if self.buf.remaining() >= 1 {
            Ok(self.buf.get_u8())
        } else {
            Err(DecodeError::UnexpectedEof)
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, DecodeError> {
        if self.buf.remaining() >= 2 {
            Ok(self.buf.get_u16())
        } else {
            Err(DecodeError::UnexpectedEof)
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        if self.buf.remaining() >= 4 {
            Ok(self.buf.get_u32())
        } else {
            Err(DecodeError::UnexpectedEof)
        }
    }

    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        let byte = self.read_u8()?;
        Ok(byte != 0)
    }

    pub fn read_long(&mut self) -> Result<(u32, u32), DecodeError> {
        let high = self.read_u32()?;
        let low = self.read_u32()?;

        Ok((high, low))
    }

    pub fn read_variable_int(&mut self, rotate: bool) -> Result<u64, DecodeError> {
        let mut result: u64 = 0;
        let mut shift = 0;

        loop {
            let mut byte = self.read_u8()?;
            if rotate && shift == 0 {
                let seventh = (byte & 0x40) >> 6;
                let msb = (byte & 0x80) >> 7;
                let n = byte << 1;

                byte = (n & !0x81) | (msb << 7) | seventh;
            }

            result |= ((byte & 0x7F) as u64) << shift;
            shift += 7;

            if byte & 0x80 == 0 {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_vint(&mut self) -> Result<i64, DecodeError> {
        let n = self.read_variable_int(true)?;
        let decoded = ((n >> 1) as i64) ^ (-((n & 1) as i64));

        Ok(decoded)
    }

    pub fn read_string(&mut self) -> Result<String, DecodeError> {
        let len = self.read_u32()?;

        if len == u32::MAX {
            return Ok(String::new());
        }

        if self.buf.remaining() < len as usize {
            return Err(DecodeError::UnexpectedEof);
        }

        let bytes = self.buf.copy_to_bytes(len as usize);

        Ok(String::from_utf8(bytes.to_vec())?)
    }

    pub fn read_data_reference(&mut self) -> Result<(i64, i64), DecodeError> {
        let high = self.read_vint()?;

        if high == 0 {
            Ok((high, 0))
        } else {
            let low = self.read_vint()?;
            Ok((high, low))
        }
    }

    pub fn read_command_header(&mut self) -> Result<Vec<i64>, DecodeError> {
        let mut header = Vec::with_capacity(9);

        for _ in 0..9 {
            header.push(self.read_vint()?);
        }

        Ok(header)
    }

    pub fn peek_int(&self) -> Result<u32, DecodeError> {
        if self.buf.remaining() >= 4 {
            let slice = self.buf.chunk();

            if slice.len() < 4 {
                let mut temp = vec![0u8; 4];
                let mut remaining = self.buf.clone();

                remaining.copy_to_slice(&mut temp);

                Ok(u32::from_be_bytes([temp[0], temp[1], temp[2], temp[3]]))
            } else {
                Ok(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
            }
        } else {
            Err(DecodeError::UnexpectedEof)
        }
    }
}
