use std::time::{SystemTime, Duration};



#[derive(Copy,Clone, Debug,Hash,PartialEq,Eq)]
pub struct Timespec{
    pub reftime: SystemTime,
    pub curtime: SystemTime,
    pub step: Duration,
}

impl Timespec{
    pub fn keytime(&self) -> u64{
        let dur = self.curtime.duration_since(self.reftime).unwrap();

        dur.as_secs()/self.step.as_secs()
    }
}