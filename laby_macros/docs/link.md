The **`<link>`** [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) element specifies relationships between the current document and an external resource.
This element is most commonly used to link to stylesheets, but is also used to establish site icons (both "favicon" style icons and icons for the home screen and apps on mobile devices) among other things.

To link an external stylesheet, you'd include a `<link>` element inside your [`head`](head!) like this:

```html
<link href="main.css" rel="stylesheet">
```

This simple example provides the path to the stylesheet inside an `href` attribute, and a `rel` attribute with a value of `stylesheet`. The `rel` stands for "relationship", and is probably one of the key features of the `<link>` element — the value denotes how the item being linked to is related to the containing document.
As you'll see from our [Link types](https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types) reference, there are many different kinds of relationship.

There are a number of other common types you'll come across. For example, a link to the site's favicon:

```html
<link rel="icon" href="favicon.ico">
```

There are a number of other icon `rel` values, mainly used to indicate special icon types for use on various mobile platforms, e.g.:

```html
<link rel="apple-touch-icon-precomposed" sizes="114x114"
      href="apple-icon-114.png" type="image/png">
```

The `sizes` attribute indicates the icon size, while the `type` contains the MIME type of the resource being linked.
These provide useful hints to allow the browser to choose the most appropriate icon available.

You can also provide a media type or query inside a `media` attribute; this resource will then only be loaded if the media condition is true. For example:

```html
<link href="print.css" rel="stylesheet" media="print">
<link href="mobile.css" rel="stylesheet" media="screen and (max-width: 600px)">
```

Some interesting new performance and security features have been added to the `<link>` element too. Take this example:

```html
<link rel="preload" href="myFont.woff2" as="font"
      type="font/woff2" crossorigin="anonymous">
```

A `rel` value of `preload` indicates that the browser should preload this resource (see [Preloading content with rel="preload"](https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types/preload) for more details), with the `as` attribute indicating the specific class of content being fetched.
The `crossorigin` attribute indicates whether the resource should be fetched with a CORS request.

Other usage notes:

- A `<link>` element can occur either in the [`head`](head!) or [`body`](body!) element, depending on whether it has a [link type](https://html.spec.whatwg.org/multipage/links.html#body-ok) that is **body-ok**.
  For example, the `stylesheet` link type is body-ok, and therefore `<link rel="stylesheet">` is permitted in the body.
  However, this isn't a good practice to follow; it makes more sense to separate your `<link>` elements from your body content, putting them in the `<head>`.
- When using `<link>` to establish a favicon for a site, and your site uses a Content Security Policy (CSP) to enhance its security, the policy applies to the favicon.
  If you encounter problems with the favicon not loading, verify that the `Content-Security-Policy` header's [`img-src` directive](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/img-src) is not preventing access to it.
- The HTML and XHTML specifications define event handlers for the `<link>` element, but it is unclear how they would be used.
- Under XHTML 1.0, empty elements such as `<link>` require a trailing slash: `<link />`.
- WebTV supports the use of the value `next` for `rel` to preload the next page in a document series.
