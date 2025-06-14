#import "deps.typ": shiroa
#import shiroa: *

#show: book


#book-meta(title: "Typstyle Documentation", summary: [
  #prefix-chapter("pages/introduction.typ")[Introduction]
  #prefix-chapter("pages/installation.typ")[Installation]
  #prefix-chapter("pages/quick-start.typ")[Quick Start]
  = Usage
  - #chapter("pages/cli-usage.typ")[Command Line]
  - #chapter("pages/features.typ")[Formatting Features]
  - #chapter("pages/escape-hatch.typ")[Escape Hatches]
  - #chapter("pages/limitations.typ")[Limitations]
  = Advanced
  - #chapter("pages/architecture.typ")[How It Works]
  = Contributing
  - #chapter("pages/dev-guide.typ")[Developer Guide]
  - #prefix-chapter("pages/changelog.typ")[Changelog]
])

// re-export page template
#import "./templates/page.typ": project, render-examples
#let book-page = project
