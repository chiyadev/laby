The **`<select>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element represents a control that provides a menu of options:

The above example shows typical `<select>` usage. It is given an `id` attribute to enable it to be associated with a [`label`](label!) for accessibility purposes, as well as a `name` attribute to represent the name of the associated data point submitted to the server. Each menu option is defined by an [`option`](option!) element nested inside the `<select>`.

Each `<option>` element should have a `value", "option` attribute containing the data value to submit to the server when that option is selected. If no `value` attribute is included, the value defaults to the text contained inside the element. You can include a `selected", "option` attribute on an `<option>` element to make it selected by default when the page first loads.

The `<select>` element has some unique attributes you can use to control it, such as `multiple` to specify whether multiple options can be selected, and `size` to specify how many options should be shown at once. It also accepts most of the general form input attributes such as `required`, `disabled`, `autofocus`, etc.

You can further nest `<option>` elements inside [`optgroup`](optgroup!) elements to create separate groups of options inside the dropdown.

For further examples, see [The native form widgets: Drop-down content](https://developer.mozilla.org/en-US/docs/Learn/Forms/Other_form_controls#drop-down_controls).
