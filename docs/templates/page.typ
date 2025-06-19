// This is important for shiroa to produce a responsive layout
// and multiple targets.
#import "../deps.typ": hypraw, shiroa
#import shiroa: (
  get-page-width, is-html-target, is-pdf-target, is-web-target, plain-text, shiroa-sys-target, target, templates,
)
#import templates: *
#import "components/mod.typ": *

// Metadata
#let page-width = get-page-width()
#let is-html-target = is-html-target()
#let is-pdf-target = is-pdf-target()
#let is-web-target = is-web-target()
#let sys-is-html-target = ("target" in dictionary(std))

/// Creates an embedded block typst frame.
#let div-frame(content, attrs: (:), tag: "div") = html.elem(tag, html.frame(content), attrs: attrs)
#let span-frame = div-frame.with(tag: "span")
#let p-frame = div-frame.with(tag: "p")

// Theme (Colors)
#let (
  style: theme-style,
  is-dark: is-dark-theme,
  is-light: is-light-theme,
  main-color: main-color,
  dash-color: dash-color,
  code-extra-colors: code-extra-colors,
) = book-theme-from(toml("theme-style.toml"), xml: it => xml(it))

// Fonts
#let main-font = (
  // "Charter",
  "Source Han Serif SC",
  // "Source Han Serif TC",
  // shiroa's embedded font
  "Libertinus Serif",
)
#let code-font = (
  // "BlexMono Nerd Font Mono",
  // shiroa's embedded font
  "DejaVu Sans Mono",
)

// Sizes
#let main-size = if is-web-target {
  16pt
} else {
  10.5pt
}
#let heading-sizes = if is-web-target {
  (2, 1.5, 1.17, 1, 0.83).map(it => it * main-size)
} else {
  (26pt, 22pt, 14pt, 12pt, main-size)
}
#let list-indent = 0.5em


/// The project function defines how your document looks.
/// It takes your content and some metadata and formats it.
/// Go ahead and customize it to your liking!
#let project(title: "Typst Book", authors: (), kind: "page", body) = {
  // set basic document metadata
  set document(author: authors, title: title) if not is-pdf-target

  // set web/pdf page properties
  set page(numbering: none, number-align: center, width: page-width) if not (sys-is-html-target or is-html-target)

  // remove margins for web target
  set page(
    margin: (
      // reserved beautiful top margin
      top: 20pt,
      // reserved for our heading style.
      // If you apply a different heading style, you may remove it.
      left: 20pt,
      // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
      bottom: 0.5em,
      // remove rest margins.
      rest: 0pt,
    ),
    height: auto,
  ) if is-web-target and not is-html-target

  // Set main text
  set text(font: main-font, size: main-size, fill: main-color, lang: "en")

  // Set main spacing
  set enum(indent: list-indent * 0.618, body-indent: list-indent)
  set list(indent: list-indent * 0.618, body-indent: list-indent)
  set par(leading: 0.7em)
  set block(spacing: 0.7em * 1.5)

  // Set text, spacing for headings
  // Render a dash to hint headings instead of bolding it as well if it's for web.
  show heading: set text(weight: "regular") if is-web-target
  show heading: it => {
    let it = {
      set text(size: heading-sizes.at(it.level))
      if is-web-target {
        heading-hash(it, hash-color: dash-color)
      }
      it
    }

    block(spacing: 0.7em * 1.5 * 1.2, below: 0.7em * 1.2, it)
  }

  // link setting
  show link: set text(fill: dash-color)

  // math setting
  show math.equation: set text(weight: 400)
  show math.equation.where(block: true): it => context if shiroa-sys-target() == "html" {
    p-frame(attrs: ("class": "block-equation"), it)
  } else {
    it
  }
  show math.equation.where(block: false): it => context if shiroa-sys-target() == "html" {
    span-frame(attrs: (class: "inline-equation"), it)
  } else {
    it
  }

  // code block setting
  set raw(theme: theme-style.code-theme) if theme-style.code-theme.len() > 0
  show raw: set text(font: code-font)
  show raw.where(block: true): set block(width: 100%)
  show: hypraw.hypraw

  // Put your custom CSS here.
  context if shiroa-sys-target() == "html" {
    html.elem("style", read("styles/base.css"))
    html.elem("style", read("styles/callout.css"))
    html.elem("style", read("styles/example.css"))
    html.elem("style", read("styles/hypraw.css"))
  }

  // Main body.
  {
    set par(justify: true)
    body
  }
}

#let part-style = heading
