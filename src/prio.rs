const MAX_NICE: i32     = 19;
const MIN_NICE: i32     = -20;
const NICE_WIDTH: u8   = (MAX_NICE - MIN_NICE + 1) as u8;

pub const MAX_RR_PRIO: u8  = 100;
pub const MAX_PRIO: u8     = MAX_RR_PRIO + NICE_WIDTH;
pub const DEFAULT_PRIO: u8 = MAX_RR_PRIO + NICE_WIDTH / 2;
