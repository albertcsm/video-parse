use std::{any::Any, fmt, io::{Cursor, Write}};

use super::sps_pps_provider::SpsPpsProvider;

pub trait Nalu: fmt::Debug {
    fn write(&self, wtr: &mut dyn Write, sps_pps_provider: &dyn SpsPpsProvider);
    fn to_bytes(&self, sps_pps_provider: &dyn SpsPpsProvider) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        self.write(&mut cursor, sps_pps_provider);
        let buffer = cursor.into_inner();
        buffer
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
