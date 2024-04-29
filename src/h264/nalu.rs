use std::{any::Any, fmt, io::Write};

use super::sps_pps_provider::SpsPpsProvider;

pub trait Nalu: fmt::Display {
    fn get_payload_size(&self) -> u32;
    fn write(&self, wtr: &mut dyn Write, sps_pps_provider: &dyn SpsPpsProvider);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
