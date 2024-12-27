use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// TODO: Define the `status` method for all states
pub trait Status {
    fn status() -> &'static str;
}

impl Status for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}

impl Status for Ready {
    fn status() -> &'static str {
        "Ready"
    }
}

impl Status for Flying {
    fn status() -> &'static str {
        "Flying"
    }
}

pub struct Sleigh<T: Status> {
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

impl<T: Status> Sleigh<T> {
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

// fn main() {
//     let sleigh = Sleigh::new();
//     assert_eq!(sleigh.status(), "Empty");

//     let ready_sleigh = sleigh.load();
//     assert_eq!(ready_sleigh.status(), "Ready");

//     let flying_sleigh = ready_sleigh.take_off();
//     assert_eq!(flying_sleigh.status(), "Flying");
// }
