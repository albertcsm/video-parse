use std::fmt;

pub trait Nalu: fmt::Display {
    fn get_payload_size(&self) -> u32;
}
