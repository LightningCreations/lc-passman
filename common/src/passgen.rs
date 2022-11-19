
bitflags::bitflags! {
    #[derive(Copy,Clone)]
    pub struct CharGroups : u32 {
        const UPPER_LETTER = 0x01;
        const LOWER_LETTER = 0x02;
        const ANY_LETTER   = 0x04;
        const NUMBER       = 0x08;
        const SYMBOL       = 0x10;
        const REDUCED_SYMBOL = 0x20;
    }
}

#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq)]
pub struct PasswordRequirements{
    pub all_of: Option<CharGroups>,
    pub at_least_n_of: Option<(u32,CharGroups)>,
    pub allow: Option<CharGroups>,
    pub max_consecutive_reps: Option<u32>,
    pub max_total_reps: Option<u32>,
    pub min_size: u32,
    pub max_size: Option<u32>,

}

static ALPHA: [&str;6] = [
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwyxz",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwyxz",
    "0123456789",
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}",
    "_-"
];