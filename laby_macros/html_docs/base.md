The **`<base>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element specifies the base URL to use for all _relative_ URLs in a document. There can be only one `<base>` element in a document.

A document's used base URL can be accessed by scripts with `Node.baseURI`. If the document has no `<base>` elements, then `baseURI` defaults to `location.href`.
