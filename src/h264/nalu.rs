use std::{any::Any, fmt, fs::File};

pub trait Nalu: fmt::Display {
    fn get_payload_size(&self) -> u32;
    fn write(&self, wtr: &mut File);
    fn as_any(&self) -> &dyn Any;
}
