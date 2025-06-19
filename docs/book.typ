#import "deps.typ": shiroa
#import shiroa: *

#show: book


#book-meta(title: "Typstyle Documentation", summary: [
  #prefix-chapter("pages/introduction.typ")[Introduction]
  #prefix-chapter("pages/installation.typ")[Installation]
  #prefix-chapter("pages/quick-start.typ")[Quick Start]
  - #prefix-chapter("pages/changelog.typ")[Changelog]
  = Usage
  - #chapter("pages/cli-usage.typ")[Command Line]
  = Features
  - #chapter("pages/features.typ")[Formatting Features]
    - #chapter("pages/features/markup.typ")[Markup]
    - #chapter("pages/features/code.typ")[Code]
    - #chapter("pages/features/math.typ")[Math Equations]
    - #chapter("pages/features/table.typ")[Tables]
    - #chapter("pages/escape-hatch.typ")[Escape Hatch]
  - #chapter("pages/limitations.typ")[Limitations]
  = Advanced
  - #chapter("pages/architecture.typ")[How It Works]
  - #chapter("pages/dev-guide.typ")[Developer Guide]
])

// re-export page template
#import "./templates/page.typ": project, render-examples
#let book-page = project

#import "templates/components/callout.typ"
