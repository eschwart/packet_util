use std::fmt::Debug;

use bitflags::Flags;
use serde::{Deserialize, Serialize};

pub trait AsPacketKind: Flags + Debug {}

pub trait AsPacketSend: Serialize {}

pub trait AsPacketRecv<'a, T: AsPacketKind>: Deserialize<'a> {
    fn kind(&self) -> T;
}
