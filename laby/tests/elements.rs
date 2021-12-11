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
fn normal() {
    let n = render!(div!(class = "class", "laby"));
    assert_eq!(n, "<div class=\"class\">laby</div>");
}

#[test]
fn void() {
    let n = render!(input!(class = "class"));
    assert_eq!(n, "<input class=\"class\">");

    let n = render!(input!());
    assert_eq!(n, "<input>");
}

#[test]
fn void_nested() {
    let n = render!(div!(class = "class", input!(class = "class")));
    assert_eq!(n, "<div class=\"class\"><input class=\"class\"></div>");
}
