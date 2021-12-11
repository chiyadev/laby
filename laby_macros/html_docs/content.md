# Deprecated

The **`<content>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element—an obsolete part of the [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components) suite of technologies—was used inside of [Shadow DOM](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_shadow_DOM) as an insertion point, and wasn't meant to be used in ordinary HTML. It has now been replaced by the [`slot`](slot!) element, which creates a point in the DOM at which a shadow DOM can be inserted.

> **Note:** Though present in early draft of the specifications and implemented in several browsers, this element has been removed in later versions of the spec, and should not be used. It is documented here to assist in adapting code written during the time it was included in the spec to work with newer versions of the specification.
