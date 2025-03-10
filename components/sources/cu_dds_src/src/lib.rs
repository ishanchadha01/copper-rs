use cu29::bincode::error::IntegerType;
use cu29::clock::RobotClock;
use cu29::prelude::*;
use rustdds::*;
use rustdds::no_key::{DataReader, DataWriter, DataSample}; // We use a NO_KEY topic here
use serde::{Serialize, Deserialize};

pub struct DDSSrc<P>
where
    P: CuMsgPayload + 'static,
{
    service_name: String,
}

impl<P> Freezable for DDSSrc<P> where P: CuMsgPayload {}

impl<'cl, P> CuSrcTask<'cl> for DDSSrc<P>
where
    P: CuMsgPayload + 'cl + 'static,
{
    type Output = output_msg!('cl, P);

    fn new(config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized
    {
        Ok(Self {
            service_name: "name".to_owned()
        })
    }

    fn start(&mut self, _clock: &RobotClock) -> CuResult<()> {
        Ok(())
    }

    fn process(&mut self, _clock: &RobotClock, new_msg: Self::Output) -> CuResult<()> {
        Ok(())
    }

    fn stop(&mut self, _clock: &RobotClock) -> CuResult<()> {
        Ok(())
    }
} impl CuSrcTask for DDSSrc<P>