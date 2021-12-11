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
    let mut v = vec![];
    v.push(div!());
    v.clear();

    let n = render!(iter!(v));
    assert_eq!(n, "");
}

#[test]
fn one() {
    let v = vec![div!()];
    let n = render!(iter!(v));
    assert_eq!(n, "<div></div>");
}

#[test]
fn many() {
    let mut v = vec![];

    for _ in 0..3 {
        v.push(div!());
    }

    let n = render!(iter!(v));
    assert_eq!(n, "<div></div><div></div><div></div>");
}

#[test]
fn complex() {
    fn fibonacci(n: u32) -> u32 {
        match n {
            1 | 2 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    let n = render!(iter!((1..=5).into_iter().map(fibonacci).map(|x| span!(x))));
    assert_eq!(
        n,
        "<span>1</span><span>1</span><span>2</span><span>3</span><span>5</span>"
    );
}

#[test]
fn lines() {
    let v = [1, 2, 3, 4, 5];
    let n = render!(iter_lines!(v));
    assert_eq!(n, "1\n2\n3\n4\n5");
}

#[test]
fn custom() {
    let v = [1, 2, 3, 4, 5];
    let n = render!(iter!(", ", v));
    assert_eq!(n, "1, 2, 3, 4, 5");
}
