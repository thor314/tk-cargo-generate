//! A batteries-included library template.
// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::Result;

mod error;
mod utils;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    utils::init_logger();
    let a = 2usize;
    assert_eq!(2 + 2, 4);
  }
}
