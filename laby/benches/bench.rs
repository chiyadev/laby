//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
#![feature(test)]
extern crate test;

use laby::*;
use test::Bencher;

#[bench]
fn simple(b: &mut Bencher) {
    b.iter(|| {
        let n = html!(
            head!(title!("laby")),
            body!(class = "dark", p!("hello, world")),
        );

        let _s = render!(n);
    });
}

#[bench]
fn simple_expanded(b: &mut Bencher) {
    b.iter(|| {
        let mut buf = laby::internal::Buffer::new();
        buf.push_str("<html><head><title>laby</title></head><body class=\"dark\"><p>hello, world</p></body></html>");

        let _s = buf.into_string();
    });
}
