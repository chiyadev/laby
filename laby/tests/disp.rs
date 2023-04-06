//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use laby::*;
use std::fmt::Display;

#[test]
fn normal() {
    struct X;

    impl Display for X {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "laby")
        }
    }

    let n = render!(format_args!("{}", X));
    assert_eq!(n, "laby");
}

#[test]
fn escape() {
    struct X;

    impl Display for X {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "&&")
        }
    }

    let n = render!(format_args!("{}", X));
    assert_eq!(n, "&amp;&amp;");
}
