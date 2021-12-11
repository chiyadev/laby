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
fn flattened() {
    let n = render!(div!(span!("laby")));
    assert_eq!(n, "<div><span>laby</span></div>");
}

#[test]
fn non_flattened() {
    let nn = span!("laby");
    let n = render!(div!(nn));
    assert_eq!(n, "<div><span>laby</span></div>");
}
