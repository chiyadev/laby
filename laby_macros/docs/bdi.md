The **`<bdi>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element tells the browser's bidirectional algorithm to treat the text it contains in isolation from its surrounding text. It's particularly useful when a website dynamically inserts some text and doesn't know the directionality of the text being inserted.

Bidirectional text is text that may contain both sequences of characters that are arranged left-to-right (LTR) and sequences of characters that are arranged right-to-left (RTL), such as an Arabic quotation embedded in an English string. Browsers implement the [Unicode Bidirectional Algorithm](https://www.w3.org/International/articles/inline-bidi-markup/uba-basics) to handle this. In this algorithm, characters are given an implicit directionality: for example, Latin characters are treated as LTR while Arabic characters are treated as RTL. Some other characters (such as spaces and some punctuation) are treated as neutral and are assigned directionality based on that of their surrounding characters.

Usually, the bidirectional algorithm will do the right thing without the author having to provide any special markup but, occasionally, the algorithm needs help. That's where `<bdi>` comes in.

The `<bdi>` element is used to wrap a span of text and instructs the bidirectional algorithm to treat this text in isolation from its surroundings. This works in two ways:

- The directionality of text embedded in `<bdi>` _does not influence_ the directionality of the surrounding text.
- The directionality of text embedded in `<bdi>` _is not influenced by_ the directionality of the surrounding text.
