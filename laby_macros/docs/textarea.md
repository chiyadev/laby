The **`<textarea>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element represents a multi-line plain-text editing control, useful when you want to allow users to enter a sizeable amount of free-form text, for example a comment on a review or feedback form.

The above example demonstrates a number of features of `<textarea>`:

- An `id` attribute to allow the `<textarea>` to be associated with a [`label`](label!) element for accessibility purposes
- A `name` attribute to set the name of the associated data point submitted to the server when the form is submitted.
- `rows` and `cols` attributes to allow you to specify an exact size for the `<textarea>` to take. Setting these is a good idea for consistency, as browser defaults can differ.
- Default content entered between the opening and closing tags. `<textarea>` does not support the `value` attribute.

The `<textarea>` element also accepts several attributes common to form `<input>`s, such as `autocomplete`, `autofocus`, `disabled`, `placeholder`, `readonly`, and `required`.
