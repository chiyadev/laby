The **`<datalist>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element contains a set of [`option`](option!) elements that represent the permissible or recommended options available to choose from within other controls.

To bind the `<datalist>` element to the control, we give it a unique identifier in the [`id`](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id) attribute, and then add the [`list`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#list) attribute to the [`input`](input!) element with the same identifier as value.
Only certain types of [`input`](input!) support this behavior, and it can also vary from browser to browser.

> **Note:** The `<option>` element can store a value as internal content and in the `value` and `label` attributes. Which one will be visible in the drop-down menu depends on the browser, but when clicked, content entered into control field will always come from the `value` attribute.
