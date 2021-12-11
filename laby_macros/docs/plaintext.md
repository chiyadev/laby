# Deprecated

The **`<plaintext>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element renders everything following the start tag as raw text, ignoring any following HTML. There is no closing tag, since everything after it is considered raw text.

> **Warning:** Do not use this element.
>
> - `<plaintext>` is deprecated since HTML 2, and not all browsers implemented it. Browsers that did implement it didn't do so consistently.
> - `<plaintext>` is obsolete in HTML5; browsers that accept it may instead treat it as a [`pre`](pre!) element that still interprets HTML within.
> - If `<plaintext>` is the first element on the page (other than any non-displayed elements, like [`head`](head!)), do not use HTML at all. Instead serve a text file with the `text/plain` [MIME-type](https://developer.mozilla.org/en-US/docs/Learn/Server-side/Configuring_server_MIME_types "Properly Configuring Server MIME Types").
> - Instead of `<plaintext>`, use the [`pre`](pre!) element or, if semantically accurate (such as for inline text), the [`code`](code!) element. Escape any `<`, `>` and `&` characters, to prevent browsers inadvertently parsing content the element content as HTML.
> - A monospaced font can be applied to any HTML element via a [CSS](https://developer.mozilla.org/en-US/docs/Web/CSS) font-family style with the `monospace` generic value.
