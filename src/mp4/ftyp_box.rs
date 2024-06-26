use std::{any::Any, fmt, io::{self, Read, Write}};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{atom::Atom, four_cc::FourCC};

pub struct FtypBox {
    pub major_brand: FourCC,
    pub minor_brand: u32,
    pub compatible_brands: Vec<FourCC>,
    pub payload_size: u64
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

    fn write(&self, wtr: &mut std::fs::File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"ftyp").unwrap();
        self.major_brand.write(wtr);
        wtr.write_u32::<BigEndian>(self.minor_brand).unwrap();
        for compatible_brand in &self.compatible_brands {
            compatible_brand.write(wtr);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for FtypBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compatible_brands = self.compatible_brands.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "ftyp(major_brand={}, minor_brand={}, compatible_brands={})", self.major_brand, self.minor_brand, compatible_brands)
    }
}

impl fmt::Debug for FtypBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FtypBox")
            .field("minor_brand", &self.minor_brand)
            .field("major_brand", &self.major_brand)
            .field("compatible_brands", &self.compatible_brands)
            .finish()
    }
}
