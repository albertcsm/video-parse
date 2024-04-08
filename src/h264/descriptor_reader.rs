use std::{cmp, io::{self, Read, Seek}};

use byteorder::ReadBytesExt;

pub struct DescriptorReader<'a> {
    rdr: &'a mut (dyn Read),
    residue_bits: u8,
    residue_value: u8,
    num_read_bytes: u64
}

impl<'a> DescriptorReader<'a> {
    pub fn new(rdr: &'a mut (dyn Read)) -> Self {
        DescriptorReader {
            rdr,
            residue_bits: 0,
            residue_value: 0,
            num_read_bytes: 0
        }
    }

    pub fn read_u(&mut self, bits: u8) -> u64 {
        let mut value: u64 = 0;
        let mut remaining_bits = bits;
        while remaining_bits > 0 {
            if self.residue_bits == 0 {
                self.residue_bits = 8;
                self.residue_value = self.rdr.read_u8().unwrap();
                self.num_read_bytes += 1;
            }
            // Example:
            // residue_value  0b10110111
            // residue_bits       ~~~~~~ (6)
            // remaining_bits     ~~~~~  (5)
            // read_bits          ~~~~~  (5)
            // read_value       0b11011
            let read_bits = cmp::min(remaining_bits, self.residue_bits);
            let read_value = self.residue_value << (8 - self.residue_bits) >> (8 - read_bits);
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
        let bits = self.count_zero_bits();
        self.read_u(bits + 1) - 1
    }

    pub fn get_num_read_bytes(&self) -> u64 {
        self.num_read_bytes
    }

    fn count_zero_bits(&mut self) -> u8 {
        let mut count = 0;
        loop {
            if self.residue_bits == 0 {
                self.residue_bits = 8;
                self.residue_value = self.rdr.read_u8().unwrap();
                self.num_read_bytes += 1;
            }
            // Example:
            // residue_value  0b11000001
            // residue_bits       ~~~~~~   (6)
            // read_value       0b00000100
            // read_bits          ~~~~~    (5)
            let read_value = self.residue_value << (8 - self.residue_bits);
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