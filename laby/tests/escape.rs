//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use laby::{internal::escape_str, *};

#[test]
fn all() {
    let s = escape_str("&\"'<>");
    assert_eq!(s, "&amp;&quot;&#39;&lt;&gt;");
}

#[test]
fn unicode() {
    let s = "이건 이스케이프 처리하지마";
    assert_eq!(s, escape_str(s));

    let s = "これもエスケープするな";
    assert_eq!(s, escape_str(s));

    let s = "これち&ゃんと\"エ'スケ<ープ>できる？";
    assert_eq!(
        "これち&amp;ゃんと&quot;エ&#39;スケ&lt;ープ&gt;できる？",
        escape_str(s)
    );

    let s = "<잘 &동&작해&&서 다\"행\"이네!>ㅇ";
    assert_eq!(
        "&lt;잘 &amp;동&amp;작해&amp;&amp;서 다&quot;행&quot;이네!&gt;ㅇ",
        escape_str(s)
    );
}

#[test]
fn child() {
    let s = "<script>bad()</script>";
    let n = render!(div!(s));

    assert_eq!(n, "<div>&lt;script&gt;bad()&lt;/script&gt;</div>");
}

#[test]
fn attr() {
    let s = "<script>bad()</script>";
    let n = render!(div!(id = s));

    assert_eq!(n, "<div id=\"&lt;script&gt;bad()&lt;/script&gt;\"></div>");
}

#[test]
fn attr_name() {
    let s = "<script>bad()</script>";
    let n = render!(div!({ s } = "what"));

    assert_eq!(n, "<div &lt;script&gt;bad()&lt;/script&gt;=\"what\"></div>");
}

#[test]
fn raw_child() {
    let s = "<script>bad()</script>";
    let n = render!(div!(raw!(s)));

    assert_eq!(n, "<div><script>bad()</script></div>");
}

#[test]
fn raw_attr() {
    let s = "<script>bad()</script>";
    let n = render!(div!(id = raw!(s)));

    assert_eq!(n, "<div id=\"<script>bad()</script>\"></div>");
}

#[test]
fn raw_attr_name() {
    let s = "<script>bad()</script>";
    let n = render!(div!({ raw!(s) } = "what"));

    assert_eq!(n, "<div <script>bad()</script>=\"what\"></div>");
}
