# Deprecated

The **`<tt>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element creates inline text which is presented using the user agent's default monospace font face. This element was created for the purpose of rendering text as it would be displayed on a fixed-width display such as a teletype, text-only screen, or line printer.

The terms **non-proportional**, **monotype**, and **monospace** are used interchangeably and have the same general meaning: they describe a typeface whose characters are all the same number of pixels wide.

This element is obsolete, however. You should use the more semantically helpful [`code`](code!), [`kbd`](kbd!), [`samp`](samp!), or [`var`](var!) elements for inline text that needs to be presented in monospace type, or the [`pre`](pre!) tag for content that should be presented as a separate block.

> **Note:** If none of the semantic elements are appropriate for your use case (for example, if you need to show some content in a non-proportional font), you should consider using the [`span`](span!) element, styling it as desired using CSS. The font-family property is a good place to start.
