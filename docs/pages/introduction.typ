#import "../book.typ": *

#show: book-page.with(title: "Introduction")

= Introduction

#context if is-html-target() {
  html.elem("a", attrs: (href: "https://repology.org/project/typstyle/versions"))[
    #html.elem("img", attrs: (
      src: "https://repology.org/badge/latest-versions/typstyle.svg",
      alt: "latest packaged version(s)",
    ))
  ]
}

*typstyle* is a beautiful and reliable code formatter for #link("https://typst.app/")[Typst].

Typstyle automatically formats your Typst source code to ensure consistency and readability. It's fast, opinionated, and preserves the semantic meaning of your code while improving its appearance.

== Key Features

- *Fast*: Formats large documents (1000+ lines) under 5ms, or a document with 300 huge equations in 15ms.
- *Reliable*: Preserves semantic meaning while improving code appearance.
- *Opinionated*: Consistent style with minimal configuration.
- *Convergent*: Multiple runs produce identical results.

== How It Works

Typstyle parses your code into an Abstract Syntax Tree (AST), applies formatting rules based on Philip Wadler's pretty printing algorithms, and outputs clean, consistently formatted code.
