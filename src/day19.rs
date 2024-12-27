// 1. We have 3 states:
// - Empty
// - Ready
// - Flying

use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<T> {
    _state: PhantomData<T>,
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Sleigh {
            _state: PhantomData,
        }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            _state: PhantomData,
        }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

fn main() {
    let mut sleigh = Sleigh::new();
    sleigh.load();
}
