// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

// pretty-expanded FIXME #23616

struct S<T> {
    a: T,
    b: usize,
}

fn range_<F>(lo: usize, hi: usize, mut it: F) where F: FnMut(usize) {
    let mut lo_ = lo;
    while lo_ < hi { it(lo_); lo_ += 1; }
}

fn create_index<T>(_index: Vec<S<T>> , _hash_fn: extern fn(T) -> usize) {
    range_(0, 256, |_i| {
        let _bucket: Vec<T> = Vec::new();
    })
}

pub fn main() { }
