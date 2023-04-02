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
///     DocType::HTML,
///     html!(
///         head!(title!("laby")),
///         body!(),
///     ),
/// );
///
/// assert_eq!(n, "<!DOCTYPE html><html><head><title>laby</title></head><body></body></html>");
/// ```
pub enum DocType {
    /// Declaration type for HTML documents.
    HTML,
}

impl Render for DocType {
    fn render(self, buffer: &mut Buffer) {
        match self {
            DocType::HTML => {
                buffer.push_str("<!DOCTYPE html>");
            }
        }
    }
}
