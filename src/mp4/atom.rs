use std::{any::Any, fmt, fs::File};

pub trait Atom: fmt::Debug {
    fn get_payload_size(&self) -> u64;
    fn write(&self, wtr: &mut File);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
