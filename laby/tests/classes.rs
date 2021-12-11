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
fn only_lits() {
    let s = classes!("1", "2", "3", "4");
    assert_eq!(render!(s), "1 2 3 4");
}

#[test]
fn only_exprs() {
    let one = "1";
    let two = "2";
    let three = "3";
    let four = "4";
    let s = classes!(one, two, three, four);
    assert_eq!(render!(s), "1 2 3 4");
}

#[test]
fn escape() {
    let x = "&";
    let s = classes!("1", "&", x, "2");
    assert_eq!(render!(s), "1 &amp; &amp; 2");
}

#[test]
fn nones() {
    let none: Option<&str> = None;
    let s = classes!(none, "1", none, none, "2", none, none, none, "3");
    assert_eq!(render!(s), " 1   2    3");
}

#[test]
fn mixed() {
    let none: Option<&str> = None;
    let some = Some("some");
    let s = classes!("1", none, "2", "3", some, "4 5", none, some, none, "6", some, none);
    assert_eq!(render!(s), "1  2 3 some 4 5  some  6 some ");
}
