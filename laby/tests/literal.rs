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
fn hex() {
    let n = render!(div!(0x11));
    assert_eq!(n, "<div>17</div>");
}

#[test]
fn raw_str() {
    let n = render!(div!(r#######"laby"#######));
    assert_eq!(n, "<div>laby</div>");
}

#[test]
fn int_large() {
    let n = render!(div!(0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff));
    assert_eq!(n, "<div>340282366920938463463374607431768211455</div>");
}

#[test]
fn float_large() {
    let n = render!(div!(1.7976931348623157E+308));
    assert_eq!(n, "<div>1.7976931348623157e308</div>"); // https://github.com/dtolnay/ryu#formatting
}

#[test]
fn float_epsilon() {
    let n = render!(div!(2.2204460492503131E-16));
    assert_eq!(n, "<div>2.220446049250313e-16</div>") // https://github.com/dtolnay/ryu#formatting
}
