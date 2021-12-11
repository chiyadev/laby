The **`<audio>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element is used to embed sound content in documents. It may contain one or more audio sources, represented using the `src` attribute or the [`source`](source!) element: the browser will choose the most suitable one. It can also be the destination for streamed media, using a `MediaStream`.

The above example shows simple usage of the `<audio>` element. In a similar manner to the [`img`](img!) element, we include a path to the media we want to embed inside the `src` attribute; we can include other attributes to specify information such as whether we want it to autoplay and loop, whether we want to show the browser's default audio controls, etc.

The content inside the opening and closing `<audio></audio>` tags is shown as a fallback in browsers that don't support the element.
