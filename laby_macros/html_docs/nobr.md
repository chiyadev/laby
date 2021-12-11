# Deprecated

The **`<nobr>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element prevents the text it contains from automatically wrapping across multiple lines, potentially resulting in the user having to scroll horizontally to see the entire width of the text.

> **Warning:** Although this element is widely supported, it was _never_ standard HTML, so you shouldn't use it. Instead, use the CSS property white-space like this:

```html
<span style="white-space: nowrap;">Long line with no breaks</span>
```
