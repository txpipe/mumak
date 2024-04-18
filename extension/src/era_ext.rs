extern crate pallas;
use pallas::ledger::traverse::Era;

pub trait EraExt {
    fn from_int(value: i32) -> Option<Self> where Self: Sized;
}

impl EraExt for Era {
    fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Era::Byron),
            1 => Some(Era::Shelley),
            2 => Some(Era::Allegra),
            3 => Some(Era::Mary),
            4 => Some(Era::Alonzo),
            5 => Some(Era::Babbage),
            6 => Some(Era::Conway),
            _ => None,
        }
    }
}
