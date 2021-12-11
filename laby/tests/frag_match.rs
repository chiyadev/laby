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
fn simple_match() {
    let x = "three";
    let n = render!(frag_match!(match x {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        _ => "what",
    }));

    assert_eq!(n, "3");
}

#[test]
fn advanced_match() {
    let x = "three";
    let n = render!(frag_match!(match x {
        "one" => div!("1"),
        "two" => iter_lines!(["two"]),
        "three" => span!(3),
        "four" => Some(div!("4")),
        _ => "what",
    }));

    assert_eq!(n, "<span>3</span>");
}

#[test]
fn simple_if() {
    let x = "three";
    let n = render!(frag_match!(if x == "one" {
        "1"
    } else if x == "two" {
        "2"
    } else if x == "three" {
        "3"
    } else if x == "four" {
        "4"
    } else {
        "5"
    }));

    assert_eq!(n, "3");
}

#[test]
fn advanced_if() {
    let x = "three";
    let n = render!(frag_match!(if x == "one" {
        div!("1")
    } else if x == "two" {
        iter_lines!(["two"])
    } else if x == "three" {
        span!(3)
    } else if x == "four" {
        Some(div!("4"))
    } else {
        "what"
    }));

    assert_eq!(n, "<span>3</span>");
}
