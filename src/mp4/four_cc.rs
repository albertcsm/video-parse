use std::{fmt, io::{self, Read, Write}};

#[derive(PartialEq, Eq)]
pub struct FourCC {
    data: [u8; 4]
}

impl FourCC {
    pub fn read(rdr: &mut impl Read) -> io::Result<Self> {
        let mut data = [0; 4];
        rdr.read(&mut data)?;

        Ok(FourCC {
            data
        })
    }

    pub fn write(&self, wtr: &mut impl Write) {
        wtr.write(&self.data).unwrap();
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fourcc = std::str::from_utf8(&self.data).unwrap();
        write!(f, "{:}", fourcc)
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fourcc = std::str::from_utf8(&self.data).unwrap();
        write!(f, "{:}", fourcc)
    }
}
