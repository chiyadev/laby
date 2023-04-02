# laby

[![Crate](https://img.shields.io/crates/v/laby)][1]
[![Docs](https://img.shields.io/docsrs/laby)][2]
[![Maintainer](https://img.shields.io/badge/maintainer-luaneko-pink)][3]
[![License](https://img.shields.io/crates/l/laby)](LICENSE)
[![Issues](https://img.shields.io/github/issues/chiyadev/laby.svg)][4]
[![Contributors](https://img.shields.io/github/contributors/chiyadev/laby.svg)][5]

laby is a small macro library for writing HTML templates in Rust. [Documentation][2]

```rust
let n = html!(
  head!(
    title!("laby"),
  ),
  body!(
    p!("Hello, world!"),
  ),
);

let s = render!(DocType::HTML5, n);
```

```html
<!DOCTYPE html><html><head><title>laby</title></head><body><p>Hello, world!</p></body></html>
```

[1]: https://crates.io/crates/laby
[2]: https://docs.rs/laby
[3]: https://github.com/luaneko
[4]: https://GitHub.com/chiyadev/laby/issues
[5]: https://github.com/chiyadev/laby/graphs/contributors
