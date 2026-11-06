#![allow(dead_code)]

use bytecheck::{check_bytes, rancor::Failure, CheckBytes};

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

fn main() {
    unsafe {
        check_bytes::<Named, Failure>(&Named { field: true }).unwrap();
        check_bytes::<Tuple, Failure>(&Tuple(true)).unwrap();
        check_bytes::<Unit, Failure>(&Unit).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Unit).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Tuple(true)).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Named { field: true }).unwrap();
    }
}
