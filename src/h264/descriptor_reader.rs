use std::{cmp, io::Read};

use super::opaque_data::OpaqueData;

pub struct DescriptorReader {
    buffer: Vec<u8>,
    next_pos: usize,
    residue_bits: u8,
}

impl<'a> DescriptorReader {
    pub fn new(rdr: &'a mut (dyn Read), len: u32) -> Self {
        let mut buffer = vec![0u8; len.try_into().unwrap()];
        rdr.read_exact(&mut buffer).unwrap();
        DescriptorReader {
            buffer,
            next_pos: 0,
            residue_bits: 0
        }
    }

    pub fn read_u(&mut self, bits: u8) -> u64 {
        let mut value: u64 = 0;
        let mut remaining_bits = bits;
        while remaining_bits > 0 {
            if self.residue_bits == 0 {
                self.residue_bits = 8;
                self.next_pos += 1;
            }
            // Example:
            // residue_value  0b10110111
            // residue_bits       ~~~~~~ (6)
            // remaining_bits     ~~~~~  (5)
            // read_bits          ~~~~~  (5)
            // read_value       0b11011
            let read_bits = cmp::min(remaining_bits, self.residue_bits);
            let read_value = self.buffer[self.next_pos - 1] << (8 - self.residue_bits) >> (8 - read_bits);
            value = (value << read_bits) | u64::from(read_value);
            self.residue_bits -= read_bits;
            remaining_bits -= read_bits;
        }
        value
    }

    pub fn read_u1(&mut self) -> bool {
        self.read_u(1) > 0
    }

    pub fn read_u8(&mut self) -> u8 {
        u8::try_from(self.read_u(8)).unwrap()
    }
    
    pub fn read_u16(&mut self) -> u16 {
        u16::try_from(self.read_u(16)).unwrap()
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::try_from(self.read_u(32)).unwrap()
    }

    pub fn read_ue_v(&mut self) -> u64 {
        let bits = self.read_zero_bits();
        self.read_u(bits + 1) - 1
    }

    pub fn read_se_v(&mut self) -> i64 {
        let bits = self.read_zero_bits();
        let magnitude = if bits > 0 { i64::try_from(self.read_u(bits)).unwrap() } else { 0 };
        let sign = self.read_u1();
        if sign {
            return -magnitude;
        } else {
            return magnitude;
        }
    }

    pub fn more_rbsp_data(&self) -> bool {
        let assumed_stop_bit;
        if self.residue_bits > 0 {
            assumed_stop_bit = self.buffer[self.next_pos - 1] << (8 - self.residue_bits) >> 7;
        } else if self.next_pos < self.buffer.len() {
            assumed_stop_bit = self.buffer[self.next_pos] >> 7;
        } else {
            return false;
        }

        if assumed_stop_bit != 1 {
            return true;
        }

        if self.residue_bits > 1 {
            let assumed_zero_bits = self.buffer[self.next_pos - 1] << (9 - self.residue_bits);
            if assumed_zero_bits != 0 {
                return true;
            }
        }
        
        for i in self.next_pos..self.buffer.len() {
            if self.buffer[i] != 0 {
                return true
            }
        }

        // empty or 1 followed by padding 0s => RBSP endding, no more data
        false
    }

    pub fn read_to_end(&mut self) -> OpaqueData {
        if let Some((last_one_byte_index, last_one_bit_index)) = self.find_last_one() {
            let mut bytes = vec![];
            let mut bits_to_read: isize = isize::try_from((last_one_byte_index - self.next_pos + 1) * 8).unwrap() + isize::from(last_one_bit_index) - isize::from(8 - self.residue_bits);
            while bits_to_read > 8 {
                let value = self.read_u8();
                bytes.push(value);
                bits_to_read -= 8;
            }
            let residue_value = if bits_to_read > 0 {
                u8::try_from(self.read_u(bits_to_read.try_into().unwrap())).unwrap()
            } else {
                0
            };
            OpaqueData{
                bytes,
                residue_value,
                residue_bits: bits_to_read.try_into().unwrap()
            }
        } else {
            OpaqueData {
                bytes: vec![],
                residue_value: 0,
                residue_bits: 0
            }
        }
    }

    pub fn read_rbsp_trailing_bits(&mut self) {
        let stop_bit = self.read_u1();
        if stop_bit != true {
            panic!("Stop bit of 1 expected but 0 is read")
        }
        let zero_bits = self.read_u(self.residue_bits);
        if zero_bits != 0 {
            panic!("Zero bits expected but some 1 bit is found in trailing bits")
        }
    }

    fn find_last_one(&self) -> Option<(usize, u8)> {
        for i in (0..self.buffer.len()).rev() {
            if self.buffer[i] & 0b00001111 != 0 {
                return Some((i, if self.buffer[i] & 0b00000001 != 0 {
                    7
                } else if self.buffer[i] & 0b00000010 != 0 {
                    6
                } else if self.buffer[i] & 0b00000100 != 0 {
                    5
                } else {
                    4
                }))
            } else if self.buffer[i] != 0 {
                return Some((i, if self.buffer[i] & 0b00010000 != 0 {
                    3
                } else if self.buffer[i] & 0b00100000 != 0 {
                    2
                } else if self.buffer[i] & 0b01000000 != 0 {
                    1
                } else {
                    0
                }))
            }
        }
        None
    }

    fn read_zero_bits(&mut self) -> u8 {
        let mut count = 0;
        loop {
            if self.residue_bits == 0 {
                self.residue_bits = 8;
                self.next_pos += 1;
            }
            // Example:
            // residue_value  0b11000001
            // residue_bits       ~~~~~~   (6)
            // read_value       0b00000100
            // read_bits          ~~~~~    (5)
            let read_value = self.buffer[self.next_pos - 1] << (8 - self.residue_bits);
            let read_bits = cmp::min(self.residue_bits, match read_value {
                0b10000000..=0b11111111 => 0,
                0b01000000..=0b01111111 => 1,
                0b00100000..=0b00111111 => 2,
                0b00010000..=0b00011111 => 3,
                0b00001000..=0b00001111 => 4,
                0b00000100..=0b00000111 => 5,
                0b00000010..=0b00000011 => 6,
                0b00000001 => 7,
                0 => 8,
            });
            count += read_bits;
            self.residue_bits -= read_bits;
            if read_bits < self.residue_bits {
                break;
            }
        }
        count
    }
}