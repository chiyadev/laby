# HTML documentation files

This directory contains the documentation for all HTML tags, which are excerpts of the
[MDN HTML reference][1].

These excerpts are generated using [this script](./process.sh). The script clones the
[MDN content repository][2] into this directory, processes the necessary files and saves the
output in this directory. The processed files are then included by [rustdoc][3] during the
documentation build process. The cloned repository is ignored from version control.

MDN content is licensed under [CC-BY-SA 2.5][4].
The original license text can be viewed [here](./LICENSE).

[1]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
[2]: https://github.com/mdn/content
[3]: https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
[4]: https://github.com/mdn/content/blob/main/LICENSE.md
