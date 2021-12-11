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
fn enabled() {
    let n = render!(button!(enabled = bool!(true)));
    assert_eq!(n, "<button enabled></button>");
}

#[test]
fn disabled() {
    let n = render!(button!(enabled = bool!(false)));
    assert_eq!(n, "<button></button>");
}

#[test]
fn variable() {
    let mut x = true;
    let n = render!(button!(enabled = bool!(x)));
    assert_eq!(n, "<button enabled></button>");

    x = false;
    let n = render!(button!(enabled = bool!(x)));
    assert_eq!(n, "<button></button>");
}

#[test]
fn none() {
    let n = render!(button!(enabled = true));
    assert_eq!(n, "<button enabled=\"true\"></button>");

    let n = render!(button!(enabled = false));
    assert_eq!(n, "<button enabled=\"false\"></button>");
}

#[test]
fn weird() {
    let n = render!(button!(
        enabled = true,
        enabled = bool!(true),
        enabled = bool!(false),
        enabled = false,
    ));

    assert_eq!(
        n,
        "<button enabled=\"true\" enabled enabled=\"false\"></button>"
    );
}
