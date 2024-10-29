use std::fmt::Debug;

use bitflags::Flags;
use serde::{Deserialize, Serialize};

pub trait AsPacketKind: Flags + Clone + Copy + Debug + Send + Sync {}

pub trait AsPacketSend: Serialize + Send + Sync {}

pub trait AsPacketRecv<'a, K: AsPacketKind>: Deserialize<'a> + Send + Sync {
    fn kind(&self) -> K;
}

pub trait AsPacket<'a, K: AsPacketKind>: AsPacketSend + AsPacketRecv<'a, K> + Send + Sync
where
    Self::Kind: AsPacketKind,
{
    type Kind;
}

impl<'a, K: AsPacketKind, T: AsPacketSend + AsPacketRecv<'a, K>> AsPacket<'a, K> for T {
    type Kind = K;
}
