use std::{any::Any, fmt, fs::File};

pub trait Atom: fmt::Display {
    fn get_payload_size(&self) -> u64;
    fn write(&self, wtr: &mut File);
    fn as_any(&self) -> &dyn Any;
}
