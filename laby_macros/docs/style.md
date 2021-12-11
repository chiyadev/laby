The **`<style>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element contains style information for a document, or part of a document. It contains CSS, which is applied to the contents of the document containing the `<style>` element.

The `<style>` element must be included inside the [`head`](head!) of the document. In general, it is better to put your styles in external stylesheets and apply them using [`link`](link!) elements.

If you include multiple `<style>` and `<link>` elements in your document, they will be applied to the DOM in the order they are included in the document — make sure you include them in the correct order, to avoid unexpected cascade issues.

In the same manner as `<link>` elements, `<style>` elements can include `media` attributes that contain [media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/Media_Queries), allowing you to selectively apply internal stylesheets to your document depending on media features such as viewport width.
