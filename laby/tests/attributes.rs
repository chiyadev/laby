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
fn children_first() {
    let n = render!(div!("test", class = "class"));
    assert_eq!(n, "<div class=\"class\">test</div>");
}

#[test]
fn attr_first() {
    let n = render!(div!(class = "class", "test"));
    assert_eq!(n, "<div class=\"class\">test</div>");
}

#[test]
fn mixed() {
    let n = render!(div!("test1", class = "class", "test2", id = "id"));
    assert_eq!(n, "<div class=\"class\" id=\"id\">test1test2</div>");
}

#[test]
fn duplicates() {
    let n = render!(div!(id = "one", id = "two"));
    assert_eq!(n, "<div id=\"one\" id=\"two\"></div>");
}

#[test]
fn variable_name() {
    let s = "attr name";
    let n = render!(div!((s) = "value"));
    assert_eq!(n, "<div attr name=\"value\"></div>");
}

#[test]
fn variable_value() {
    let s = "value";
    let n = render!(div!("name" = s));
    assert_eq!(n, "<div name=\"value\"></div>");
}

#[test]
fn variable_both() {
    let k = "key";
    let v = "value";
    let n = render!(div!((k) = v));
    assert_eq!(n, "<div key=\"value\"></div>");
}

#[test]
fn variable_what() {
    // if you're reading this, please don't abuse my library like this - phos
    let n = render!(div!((div!({ span!("laby") } = "what?")) = "what??"));
    assert_eq!(
        n,
        "<div <div <span>laby</span>=\"what?\"></div>=\"what??\"></div>"
    );
}
