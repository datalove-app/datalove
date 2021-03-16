mod interfaces;
mod v0;

use sha2::Sha256;

///
pub enum Clock {
    v0 {
        clock: v0::BloomClock<u8, Sha256, 2, 8>,
        // var: v0::BloomClockVar<UInt8, _, 2, 8>,
    },
}

///
impl Clock {
    ///
    fn handle_op<'a>(&'a mut self, op: Operation<'a>) {
        unimplemented!()
    }

    ///
    fn prove_op<'a>(&'a mut self, op: Operation<'a>) {
        unimplemented!()
    }
}

///
pub enum Operation<'a> {
    Increment(&'a [u8]),
    Synchronize(Clock),
}
