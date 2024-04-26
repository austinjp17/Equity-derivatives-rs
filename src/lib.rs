// #![allow(unused_imports)]

pub mod structs;
pub mod mocking;
// pub mod structs;
pub mod helpers;

#[derive(Clone, Copy)]
pub enum OptionType {
    Call,
    Put
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}




