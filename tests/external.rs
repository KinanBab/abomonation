#![feature(const_refs_to_cell)]

extern crate abomonation;
extern crate derive;

use derive::external_abomoniate;
use abomonation::log::external_log;

mod test_m {
  use std::fmt::Display;

  pub struct MyStruct<T: Display, P> {
      x: T,
      y: String,
      z: P,
  }

  impl<T: Display, P> MyStruct<T, P> {
      pub fn new(x: T, y: String, z: P) -> Self {
          MyStruct {
            x,
            y,
            z,
          }
      }
  }
}

external_abomoniate!(test_m::MyStruct, <T: ::std::fmt::Display, P>, (T, String, P), true);


#[test]
fn test() {
    let obj = test_m::MyStruct::new(16737u16, String::from("hello"), 1113736035u32);
    let string = external_log::<MyStructAbomonated<_, _>>(&obj);
    assert!(string.contains("aA"));
    assert!(string.contains("cCbB"));
    assert!(string.contains("hello"));
}
