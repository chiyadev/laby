//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use laby::*;

#[test]
fn none() {
    let n = render!();
    assert_eq!(n, "");
}

#[test]
fn single() {
    let n = render!(div!());
    assert_eq!(n, "<div></div>");
}

#[test]
fn multiple() {
    let n = render!(div!(), span!(), button!());
    assert_eq!(n, "<div></div><span></span><button></button>");
}
