use std::{any::Any, fmt};

pub trait Nalu: fmt::Display {
    fn get_payload_size(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}
