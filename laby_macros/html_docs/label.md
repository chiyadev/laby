The **`<label>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element represents a caption for an item in a user interface.

Associating a `<label>` with a form control, such as [`input`](input!) or [`textarea`](textarea!) offers some major advantages:

- The label text is not only visually associated with its corresponding text input; it is programmatically associated with it too. This means that, for example, a screen reader will read out the label when the user is focused on the form input, making it easier for an assistive technology user to understand what data should be entered.
- When a user clicks or touches/taps a label, the browser passes the focus to its associated input (the resulting event is also raised for the input). That increased hit area for focusing the input provides an advantage to anyone trying to activate it — including those using a touch-screen device.

To explicitly associate a `<label>` element with an `<input>` element, you first need to add the `id` attribute to the `<input>` element. Next, you add the `for` attribute to the `<label>` element, where the value of `for` is the same as the `id` in the `<input>` element.

Alternatively, you can nest the `<input>` directly inside the `<label>`, in which case the `for` and `id` attributes are not needed because the association is implicit:

```html
<label>
  Do you like peas?
  <input type="checkbox" name="peas" />
</label>
```ignore

The form control that a label is labeling is called the _labeled control_ of the label element. Multiple labels can be associated with the same form control:

```html
<label for="username">Enter your username:</label>
<input id="username" name="username" type="text" />
<label for="username">Forgot your username?</label>
```

Elements that can be associated with a `<label>` element include [`button`](button!), [`input`](input!) (except for `type="hidden"`), [`meter`](meter!), [`output`](output!), [`progress`](progress!), [`select`](select!) and [`textarea`](textarea!).
