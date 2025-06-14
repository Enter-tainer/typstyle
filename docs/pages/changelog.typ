#import "../book.typ": *

#show: book-page.with(title: "Changelog")

#import "../deps.typ": cmarker

#cmarker.render(read("../../CHANGELOG.md"))
