The **`<details>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element creates a disclosure widget in which information is visible only when the widget is toggled into an "open" state. A summary or label must be provided using the [`summary`](summary!) element.

A disclosure widget is typically presented onscreen using a small triangle which rotates (or twists) to indicate open/closed status, with a label next to the triangle. The contents of the `<summary>` element are used as the label for the disclosure widget.

A `<details>` widget can be in one of two states. The default _closed_ state displays only the triangle and the label inside `<summary>` (or a details attribute. Unfortunately, at this time there's no built-in way to animate the transition between open and closed.

By default when closed, the widget is only tall enough to display the disclosure triangle and summary. When open, it expands to display the details contained within.

Fully standards-compliant implementations automatically apply the CSS `display: list-item` to the [`summary`](summary!) element. You can use this to customize its appearance further. See Customizing the disclosure widget for further details.
