The **`<label>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element represents a caption for an item in a user interface.

Associating a `<label>` with an [`input`](input!) element offers some major advantages:

- The label text is not only visually associated with its corresponding text input; it is programmatically associated with it too. This means that, for example, a screen reader will read out the label when the user is focused on the form input, making it easier for an assistive technology user to understand what data should be entered.
- When a user clicks or touches/taps a label, the browser passes the focus to its associated input (the resulting event is also raised for the input). That increased hit area for focusing the input provides an advantage to anyone trying to activate it — including those using a touch-screen device.

To associate the `<label>` with an `<input>` element, you need to give the `<input>` an `id` attribute. The `<label>` then needs a `for` attribute whose value is the same as the input's `id`.

Alternatively, you can nest the `<input>` directly inside the `<label>`, in which case the `for` and `id` attributes are not needed because the association is implicit:

```html
<label>Do you like peas?
  <input type="checkbox" name="peas">
</label>
```

The form control that a label is labeling is called the _labeled control_ of the label element. Multiple labels can be associated with the same form control:

```html
<label for="username">Enter your username:</label>
<input id="username">
<label for="username">Forgot your username?</label>
```
