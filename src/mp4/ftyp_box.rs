use std::{fmt, io::{self, Read}};
use byteorder::{BigEndian, ReadBytesExt};

use super::{atom::Atom, four_cc::FourCC};

pub struct FtypBox {
    major_brand: FourCC,
    minor_brand: u32,
    compatible_brands: Vec<FourCC>,
    payload_size: u64
}

impl FtypBox {
    pub fn read(rdr: &mut impl Read, len: u64) -> io::Result<Self> {
        let major_brand = FourCC::read(rdr)?;
        let minor_brand = rdr.read_u32::<BigEndian>()?;
        let num_compatible_brands = (len - 8) / 4;
        let mut compatible_brands = Vec::new();
        for _i in 0..num_compatible_brands {
            let compatible_brand = FourCC::read(rdr)?;
            compatible_brands.push(compatible_brand);
        }

        Ok(FtypBox {
            major_brand,
            minor_brand,
            compatible_brands,
            payload_size: len
        })
    }
}

impl Atom for FtypBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl fmt::Display for FtypBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compatible_brands = self.compatible_brands.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "ftyp(major_brand={}, minor_brand={}, compatible_brands={})", self.major_brand, self.minor_brand, compatible_brands)
    }
}
