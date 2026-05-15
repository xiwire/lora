use std::{any::Any, collections::HashSet};

pub trait ProcessingUnit { 
    fn process(&self) -> ();
}

pub struct UnitParam<T: Default> {
    value: T,
}

impl<T: Default> UnitParam<T> {
    pub fn new() -> Self {
        UnitParam { value: T::default() }
    }

    pub fn init(value: T) -> Self {
        UnitParam { value: value }
    }
}

pub struct UnitParamSet {
    params: HashSet<String, Box<dyn Any>>,
}

impl UnitParamSet {

}
