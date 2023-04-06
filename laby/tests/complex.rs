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
fn nesting() {
    let title = title!("laby");
    let head = head!(title);
    let attr = "laby";
    let input = input!(class = attr);
    let form = form!(input);
    let body = body!(form);
    let html = html!(head, body);
    let n = render!(html);

    assert_eq!(
        n,
        "<html><head><title>laby</title></head><body><form><input class=\"laby\"></form></body></html>"
    );
}

#[test]
fn argument() {
    fn comp(n: impl Render) -> impl Render {
        div!(n)
    }

    let n = render!(comp(comp(comp(comp(comp(span!()))))));
    assert_eq!(
        n,
        "<div><div><div><div><div><span></span></div></div></div></div></div>"
    );
}

#[test]
fn types() {
    fn comp() -> impl Render {
        enum X {
            One,
            Two,
            Three,
        }

        use X::*;

        impl Render for X {
            fn render(self, buf: &mut internal::Buffer) {
                match self {
                    One => div!("one").render(buf),
                    Two => span!("two").render(buf),
                    Three => button!("three").render(buf),
                }
            }
        }

        iter!([One, Two, Three, Two, One])
    }

    let n = render!(comp());
    assert_eq!(
        n,
        "<div>one</div><span>two</span><button>three</button><span>two</span><div>one</div>"
    );
}
