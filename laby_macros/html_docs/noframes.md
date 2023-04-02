# Deprecated

The **`<noframes>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element provides content to be presented in browsers that don't support (or have disabled support for) the [`frame`](frame!) element. Although most commonly-used browsers support frames, there are exceptions, including certain special-use browsers including some mobile browsers, as well as text-mode browsers.

A `<noframes>` element can contain any HTML elements that are allowed within the body of an HTML document, except for the [`frameset`](frameset!) and [`frame`](frame!) elements, since using frames when they aren't supported doesn't make sense.

`<noframes>` can be used to present a message explaining that the user's browser doesn't support frames, but ideally should be used to present an alternate form of the site that doesn't use frames but still offers the same or similar functionality.

> **Note:** This element is obsolete and shouldn't be used, since the [`frame`](frame!) and [`frameset`](frameset!) elements are also obsolete. When frames are needed at all, they should be presented using the [`iframe`](iframe!) element.
