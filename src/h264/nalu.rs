use std::{any::Any, fmt, io::Write};

pub trait Nalu: fmt::Display {
    fn get_payload_size(&self) -> u32;
    fn write(&self, wtr: &mut dyn Write);
    fn as_any(&self) -> &dyn Any;
}
