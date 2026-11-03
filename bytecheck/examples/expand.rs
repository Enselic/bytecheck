#![allow(dead_code)]

use bytecheck::CheckBytes;

#[derive(CheckBytes)]
#[repr(C)]
struct Named {
    field: bool,
}

#[derive(CheckBytes)]
#[repr(C)]
struct Tuple(bool);

#[derive(CheckBytes)]
struct Unit;

#[derive(CheckBytes)]
#[repr(u8)]
enum Choice {
    Unit,
    Tuple(bool),
    Named { field: bool },
}

fn main() {}
