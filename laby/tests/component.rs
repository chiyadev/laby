//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
#![feature(decl_macro)]
use laby::*;

#[test]
fn simple() {
    #[laby]
    fn test() -> impl Render {
        div!("test")
    }

    let n = render!(test!());
    assert_eq!(n, "<div>test</div>");
}

#[test]
fn args() {
    #[laby]
    fn test<'a>(a: &'a str, b: String, c: usize) -> impl Render + 'a {
        div!(a, ", ", b, ", ", c)
    }

    let n = render!(test!(a = "one", c = 3, b = "two".into()));
    assert_eq!(n, "<div>one, two, 3</div>");
}

#[test]
fn difficult_args() {
    #[laby]
    fn test<'a, T: Render + 'a, U: Render + 'a, V: Render + 'a>(
        a: &'a str,
        b: &'a [&'a str],
        c: T,
        d: impl Render + 'a,
        e: (U, V),
    ) -> impl Render + 'a {
        div!(
            a,
            ", ",
            iter!(b.iter().map(|s| *s)),
            ", ",
            c,
            ", ",
            d,
            ", ",
            e.0,
            ", ",
            e.1
        )
    }

    let n = render!(test!(
        b = &["two"],
        d = frag!("four"),
        a = "one",
        e = (5, 6),
        c = 3,
    ));

    assert_eq!(n, "<div>one, two, 3, four, 5, 6</div>");
}

#[test]
fn default_args() {
    #[laby]
    fn test(req: String, opt: Option<String>, #[default] def: Option<String>) -> impl Render {
        div!(req, ", ", opt, ", ", def)
    }

    let n = render!(test!(
        def = Some("def".into()),
        req = "req".into(),
        opt = Some("opt".into()),
    ));

    assert_eq!(n, "<div>req, opt, def</div>");

    let n = render!(test!(
        opt = None,
        req = "req".into(),
        def = Some("def".into()),
    ));

    assert_eq!(n, "<div>req, , def</div>");

    let n = render!(test!(req = "req".into(), opt = None));
    assert_eq!(n, "<div>req, , </div>");
}

#[test]
fn direct_call() {
    #[laby]
    fn test(arg: &'static str) -> impl Render {
        div!(arg)
    }

    let n = render!(test!(arg = "named"));
    assert_eq!(n, "<div>named</div>");

    let n = render!(test("direct"));
    assert_eq!(n, "<div>direct</div>");
}

#[test]
fn modules() {
    mod nested {
        pub mod nested {
            pub mod nested {
                #[laby::laby]
                #[allow(dead_code)]
                fn hidden() {}

                #[laby::laby]
                pub fn visible() {}
            }
        }
    }

    use nested::nested::nested::*;

    visible();
    visible!();

    //hidden();
}
