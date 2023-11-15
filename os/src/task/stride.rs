use core::cmp;

#[derive(Debug, Clone, Copy)]
pub struct Stride {
    is_overflow: bool,
    value: u8,
}

impl Stride {
    pub fn new(value: u8) -> Self {
        Self {
            is_overflow: false,
            value
        }
    }

    pub fn step(&mut self, prio: u8) {
        let pass = BIG_STRIDE / prio;
        let (value, is_overflow) = self.value.overflowing_add(pass);
        self.value = value;
        self.is_overflow = is_overflow;
    }
}

pub const BIG_STRIDE: u8 = 255;


impl Ord for Stride {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.is_overflow, other.is_overflow) {
            (true, true) | (false, false) => self.value.cmp(&other.value),
            (true, false) => cmp::Ordering::Greater,
            (false, true) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stride {
    // never equal
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for Stride {
    
}