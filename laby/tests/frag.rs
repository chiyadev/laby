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
fn single() {
    let n = render!(div!(frag!(span!("test"))));
    assert_eq!(n, "<div><span>test</span></div>");
}

#[test]
fn multi() {
    let n = render!(div!(frag!(span!("one"), span!("two"))));
    assert_eq!(n, "<div><span>one</span><span>two</span></div>");
}

#[test]
fn multi2() {
    let n = render!(frag!(div!(), span!(), input!(), div!()));
    assert_eq!(n, "<div></div><span></span><input><div></div>");
}

#[test]
fn none() {
    let n = render!(div!(frag!()));
    assert_eq!(n, "<div></div>");
}

#[test]
fn none2() {
    let n = render!(frag!());
    assert_eq!(n, "");
}

#[test]
fn types() {
    let n = render!(div!(frag!(span!("one"), div!("two"))));
    assert_eq!(n, "<div><span>one</span><div>two</div></div>");
}
