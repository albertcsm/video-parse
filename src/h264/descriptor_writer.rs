use std::{cmp, io::Write};

use byteorder::{BigEndian, WriteBytesExt};

pub struct DescriptorWriter<'a> {
    wtr: &'a mut (dyn Write),
    buffer: Vec<u8>,
    residue_bits: u8,
    residue_value: u8
}

impl<'a> DescriptorWriter<'a> {
    pub fn new(wtr: &'a mut (dyn Write)) -> Self {
        DescriptorWriter {
            wtr,
            buffer: vec![],
            residue_bits: 0,
            residue_value: 0
        }
    }

    pub fn append_u(&mut self, bits: u8, value: u64) {
        let mut remaining_bits = bits;
        let mut shifted_value: u64 = value;
        // Example:
        // residue_value = 0b11111100
        // residue_bits  =   ~~~~~~      (6)
        // value         =       0b10101
        // bits          =         ~~~~~ (5)
        // write_bits    =         ~~    (2)
        while remaining_bits > 0 {
            let write_bits = cmp::min(8 - self.residue_bits, remaining_bits);
            let write_value = shifted_value >> (remaining_bits - write_bits) << (8 - self.residue_bits - write_bits);
            self.residue_value = self.residue_value | (write_value as u8);
            self.residue_bits += write_bits;
            shifted_value = (shifted_value.overflowing_shl((64 - remaining_bits + write_bits).into()).0).overflowing_shr((64 - remaining_bits + write_bits).into()).0;
            remaining_bits -= write_bits;
            if self.residue_bits == 8 {
                self.buffer.push(self.residue_value);
                self.residue_value = 0;
                self.residue_bits = 0;
            }
        }
    }

    pub fn append_u1(&mut self, value: bool) {
        self.append_u(1, value.into());
    }

    pub fn append_u8(&mut self, value: u8) {
        self.append_u(8, value.into());
    }

    pub fn append_u16(&mut self, value: u16) {
        self.append_u(16, value.into());
    }

    pub fn append_u32(&mut self, value: u32) {
        self.append_u(32, value.into());
    }

    pub fn append_ue_v(&mut self, value: u64) {
        // 0 -> 1
        // 1 -> 010
        // 2 -> 011
        // 3 -> 00100
        let bits = DescriptorWriter::count_bits(value + 1);
        self.append_u(bits - 1, 0);
        self.append_u(bits, value + 1);
    }

    pub fn append_all(&mut self, values: &Vec<u8>) {
        if self.residue_bits != 0 {
            panic!("There are residue bits in write buffer, not ready to append_all");
        }
        self.buffer.write_all(&values).unwrap();
    }

    pub fn write_with_size_and_header(&mut self, header: u8) {
        let len = self.buffer.len() + 1;
        self.wtr.write_u32::<BigEndian>(len.try_into().unwrap()).unwrap();
        self.wtr.write_u8(header).unwrap();
        self.wtr.write_all(&self.buffer).unwrap();
        self.buffer.clear();
    }

    fn count_bits(value: u64) -> u8 {
        match value {
            0 => 0,
            0b1 => 1,
            0b10..=0b11 => 2,
            0b100..=0b111 => 3,
            0b1000..=0b1111 => 4,
            0b10000..=0b11111 => 5,
            0b100000..=0b111111 => 6,
            0b1000000..=0b1111111 => 7,
            0b10000000..=0b11111111 => 8,
            _ => todo!()
        }
    }
}
