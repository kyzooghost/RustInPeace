// MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri run

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr;

struct Unmovable {
  data: String,
  slice: *mut String,
  _pin: PhantomPinned,
}

impl Unmovable {
  // To ensure the data doesn't move when the function returns,
  // we place it in the heap where it will stay for the lifetime of the object,
  // and the only way to access it would be through a pointer to it.
  fn new(data: String) -> Pin<Box<Self>> {
      let res = Unmovable {
          data,
          // we only create the pointer once the data is in place
          // otherwise it will have already moved before we even started
          slice: ptr::null_mut(),
          _pin: PhantomPinned,
      };

      let mut boxed = Box::pin(res);

      let slice = &boxed.data as *const String as *mut String;

      // we know this is safe because modifying a field doesn't move the whole struct
      unsafe {
          let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
          Pin::get_unchecked_mut(mut_ref).slice = slice;
      }
      boxed
  }
}

fn main () {
  let mut unmoved = Unmovable::new("hello".to_string());

  let mut new_string = "a".to_string();
  let ptr: *mut String = &mut new_string;

  unsafe {
    println!("{:?}", *ptr);
    println!("{:?}", *unmoved.slice);
  }

  unsafe {
    let mut_ref: Pin<&mut Unmovable> = Pin::as_mut(&mut unmoved);
    Pin::get_unchecked_mut(mut_ref).slice = ptr;
  }

  unsafe {
    println!("{:?}", *ptr);
    println!("{:?}", *unmoved.slice);
  }
}

