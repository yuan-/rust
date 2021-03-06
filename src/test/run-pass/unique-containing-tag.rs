// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

pub fn main() {
    enum t { t1(int), t2(int), }

    let _x: Box<_> = box t::t1(10);

    /*alt *x {
      t1(a) {
        assert_eq!(a, 10);
      }
      _ { panic!(); }
    }*/

    /*alt x {
      box t1(a) {
        assert_eq!(a, 10);
      }
      _ { panic!(); }
    }*/
}
