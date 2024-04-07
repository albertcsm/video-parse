use std::fmt;

pub trait Atom: fmt::Display {
    fn get_payload_size(&self) -> u64;
}
