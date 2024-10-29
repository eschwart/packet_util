use std::fmt::Debug;

use bitflags::Flags;
use serde::{Deserialize, Serialize};

pub trait AsPacketKind: Flags + Debug {}

pub trait AsPacketSend: Serialize {}

pub trait AsPacketRecv<'a, K: AsPacketKind>: Deserialize<'a> {
    fn kind(&self) -> K;
}

pub trait AsPacket<'a, K: AsPacketKind>: AsPacketSend + AsPacketRecv<'a, K> {
    type Kind;
}

impl<'a, K: AsPacketKind, T: AsPacketSend + AsPacketRecv<'a, K>> AsPacket<'a, K> for T {
    type Kind = K;
}
