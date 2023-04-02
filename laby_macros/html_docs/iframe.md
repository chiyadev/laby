The **`<iframe>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element represents a nested browsing context, embedding another HTML page into the current one.

Each embedded browsing context has its own [session history](https://developer.mozilla.org/en-US/docs/Web/API/History) and [document](https://developer.mozilla.org/en-US/docs/Web/API/Document). The browsing context that embeds the others is called the _parent browsing context_. The _topmost_ browsing context — the one with no parent — is usually the browser window, represented by the `Window` object.

> **Warning:** Because each browsing context is a complete document environment, every `<iframe>` in a page requires increased memory and other computing resources. While theoretically you can use as many `<iframe>`s as you like, check for performance problems.
