use syscall::error::Result;
use syscall::io::{Dma, Io, Mmio};

use super::ring::Ring;
use super::trb::Trb;

#[repr(packed)]
pub struct EventRingSte {
    pub address: Mmio<u64>,
    pub size: Mmio<u16>,
    _rsvd: Mmio<u16>,
    _rsvd2: Mmio<u32>,
}

pub struct EventRing {
    pub ste: Dma<EventRingSte>,
    pub ring: Ring,
}

impl EventRing {
    pub fn new() -> Result<EventRing> {
        let mut ring = EventRing {
            ste: Dma::zeroed()?,
            ring: Ring::new(false)?,
        };

        ring.ste.address.write(ring.ring.trbs.physical() as u64);
        ring.ste.size.write(ring.ring.trbs.len() as u16);

        Ok(ring)
    }

    pub fn next(&mut self) -> &mut Trb {
        self.ring.next().0
    }
}
