The **`<img>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element embeds an image into the document.

The above example shows usage of the `<img>` element:

- The `src` attribute is **required**, and contains the path to the image you want to embed.
- The `alt` attribute holds a text description of the image, which isn't mandatory but is **incredibly useful** for accessibility — screen readers read this description out to their users so they know what the image means. Alt text is also displayed on the page if the image can't be loaded for some reason: for example, network errors, content blocking, or linkrot.

There are many other attributes to achieve various purposes:

- [Referrer](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy)/img and `referrerpolicy", "img`.
- Use both `width", "img` and `height", "img` to set the intrinsic size of the image, allowing it to take up space before it loads, to mitigate content layout shifts.
- Responsive image hints with `sizes", "img` and `srcset", "img` (see also the [`picture`](picture!) element and our [Responsive images](https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images) tutorial).
