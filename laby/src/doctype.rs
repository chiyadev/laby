//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use laby_common::{internal::Buffer, Render};

/// Renders a document type declaration.
///
/// This type can be used together with the [`render!`](crate::render) macro to generate a valid
/// HTML document.
///
/// # Example
///
/// ```
/// # use laby::*;
/// let n = render!(
///     DocType::HTML5,
///     html!(
///         head!(title!("laby")),
///         body!(),
///     ),
/// );
///
/// assert_eq!(n, "<!DOCTYPE html><html><head><title>laby</title></head><body></body></html>");
/// ```
pub enum DocType {
    /// Declaration for an HTML5 document.
    HTML5,
}

impl Render for DocType {
    fn render(self, buf: &mut Buffer) {
        match self {
            DocType::HTML5 => {
                buf.push_str("<!DOCTYPE html>");
            }
        }
    }
}
