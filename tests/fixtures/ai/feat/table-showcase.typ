
#set page(margin: 0.8in)
#set text(size: 10pt)

= Table and Grid Formatting Excellence

== Basic Table with Column-Aware Formatting

// Simple table that demonstrates column awareness
#table(
  columns: (auto, 1fr, auto, auto),
  stroke: 0.5pt,
  fill: (col, row) => if row == 0 { rgb("#e8f4fd") },

  [*Product*], [*Description*], [*Price*], [*Stock*],
  [Laptop], [High-performance computing device with advanced graphics], [\$1,299], [15],
  [Mouse], [Wireless optical mouse with ergonomic design], [\$29], [67],
  [Keyboard], [Mechanical keyboard with RGB backlighting], [\$149], [23],
  [Monitor], [4K Ultra HD display with HDR support], [\$449], [8],
  [Headphones], [Noise-canceling over-ear headphones], [\$199], [31]
)

== Complex Grid with Mixed Content

#grid(
  columns: (1fr, 2fr, 1fr),
  rows: (auto, auto, 1fr, auto),
  stroke: (x, y) => (
    left: if x > 0 { 0.5pt },
    top: if y > 0 { 0.5pt }
  ),
  fill: (col, row) => {
    if row == 0 { rgb("#f0f0f0") }
    else if col == 1 { rgb("#f8f8f8") }
  },

  // Header row
  grid.cell(colspan: 3, fill: rgb("#d1ecf1"))[
    #align(center)[*Quarterly Sales Report - Q4 2024*]
  ],

  // Second row with individual cells
  [*Region*],
  [*Performance Metrics*],
  [*Actions*],

  // Data rows with complex content
  [
    === North America
    - United States
    - Canada
    - Mexico
  ],
  [
    #table(
      columns: (auto, auto),
      stroke: none,
      [Revenue:], [\$2.4M],
      [Growth:], [+12.5%],
      [Units:], [4,832],
      [Satisfaction:], [94.2%]
    )
  ],
  [
    #align(center)[
      ✅ Target Met \
      📈 Expand Marketing \
      🎯 Q1 Focus
    ]
  ],

  [
    === Europe
    - United Kingdom
    - Germany
    - France
    - Spain
  ],
  [
    #table(
      columns: (auto, auto),
      stroke: none,
      [Revenue:], [\$1.8M],
      [Growth:], [+8.3%],
      [Units:], [3,621],
      [Satisfaction:], [91.7%]
    )
  ],
  [
    #align(center)[
      ⚠️ Below Target \
      💡 New Strategy \
      🔄 Process Review
    ]
  ]
)

== Table with Headers and Footers

#table(
  columns: (auto, auto, auto, auto, auto),
  stroke: 0.5pt,
  fill: (col, row) => {
    if row == 0 or row == 1 { rgb("#e3f2fd") }
    else if row >= 8 { rgb("#f3e5f5") }
    else if calc.rem(row, 2) == 0 { rgb("#f9f9f9") }
  },

  // Multi-level header
  table.header(
    table.cell(colspan: 5, fill: rgb("#1976d2"), text(white)[
      #align(center)[*Employee Performance Dashboard*]
    ]),
    [*Name*], [*Department*], [*Performance*], [*Projects*], [*Rating*]
  ),

  // Data rows
  [Alice Johnson], [Engineering], [95%], [5], [⭐⭐⭐⭐⭐],
  [Bob Smith], [Marketing], [87%], [3], [⭐⭐⭐⭐],
  [Carol Davis], [Design], [92%], [4], [⭐⭐⭐⭐⭐],
  [David Wilson], [Sales], [89%], [6], [⭐⭐⭐⭐],
  [Emma Brown], [Engineering], [96%], [4], [⭐⭐⭐⭐⭐],
  [Frank Miller], [Marketing], [83%], [2], [⭐⭐⭐],

  // Footer with summary
  table.footer(
    table.cell(colspan: 2, fill: rgb("#9c27b0"), text(white)[
      *Summary Statistics*
    ]),
    table.cell(fill: rgb("#9c27b0"), text(white)[*Avg: 90.3%*]),
    table.cell(fill: rgb("#9c27b0"), text(white)[*Total: 24*]),
    table.cell(fill: rgb("#9c27b0"), text(white)[*Excellent*])
  )
)

== Advanced Grid Layouts

// Complex grid with spanning cells and varied content
#table(
  columns: (1fr, 1fr, 1fr, 1fr),
  rows: (auto, auto, auto, auto),
  stroke: 1pt + rgb("#666"),
  fill: (col, row) => {
    let colors = (rgb("#ffebee"), rgb("#e8f5e8"), rgb("#e3f2fd"), rgb("#fff3e0"))
    colors.at(calc.rem(col + row, 4))
  },

  // Spanning header
  table.cell(colspan: 4, fill: rgb("#333"), text(white)[
    #align(center)[*Technology Stack Comparison*]
  ]),

  // Category headers
  [*Frontend*], [*Backend*], [*Database*], [*DevOps*],

  // Technology options
  table.cell(rowspan: 2)[
    *React Ecosystem*
    - React 18
    - Next.js 14
    - TypeScript
    - Tailwind CSS
    - Framer Motion
  ],
  [
    *Node.js Stack*
    - Express.js
    - Fastify
    - NestJS
    - Socket.io
  ],
  [
    *SQL Databases*
    - PostgreSQL
    - MySQL
    - SQLite
  ],
  table.cell(rowspan: 2)[
    *Cloud Native*
    - Docker
    - Kubernetes
    - AWS/Azure
    - GitHub Actions
    - Terraform
  ],

  [
    *Python Stack*
    - FastAPI
    - Django
    - Flask
    - Celery
  ],
  [
    *NoSQL Options*
    - MongoDB
    - Redis
    - Elasticsearch
  ]
)

== Table with Complex Data Processing

#let generate-metrics = (a) => ()
#let departments = ("Engineering", "Design", "Marketing", "Sales", "HR", "Finance", "Operations")
#let metrics = generate-metrics(departments)

#figure(
  table(
    columns: (auto, auto, auto, auto, auto, auto),
    stroke: (x, y) => {
      if y == 0 { (bottom: 2pt + rgb("#333")) }
      else if y == metrics.len() + 1 { (top: 2pt + rgb("#333")) }
      else { (bottom: 0.5pt + rgb("#ccc")) }
    },
    fill: (col, row) => {
      if row == 0 { rgb("#e1f5fe") }
      else if metrics.at(row - 1).performance >= 95 { rgb("#e8f5e8") }
      else if metrics.at(row - 1).performance >= 90 { rgb("#fff3e0") }
      else if metrics.at(row - 1).performance < 85 { rgb("#ffebee") }
    },

    [*Department*], [*Performance %*], [*Projects*], [*Satisfaction*], [*Trend*], [*Status*],

    ..metrics.map(m => (
      m.department,
      str(m.performance) + "%",
      str(m.projects),
      str(m.satisfaction) + "%",
      m.trend,
      if m.performance >= 95 { "Excellent" }
      else if m.performance >= 90 { "Good" }
      else if m.performance >= 85 { "Average" }
      else { "Needs Improvement" }
    )).flatten()
  ),
  caption: [Department Performance Metrics with Automated Formatting],
  placement: auto
)

== Grid with Mathematical Content

#grid(
  columns: (1fr, 1fr),
  stroke: 0.5pt,
  fill: (col, row) => if calc.rem(col + row, 2) == 0 { rgb("#f8f8f8") },

  [
    *Quadratic Formula*
    $ x = frac(-b plus.minus sqrt(b^2 - 4a c), 2a) $

    Where:
    - $a$, $b$, $c$ are coefficients
    - $b^2 - 4a c$ is the discriminant
  ],

  [
    *Integration by Parts*
    $ integral u dif v = u v - integral v dif u $

    Steps:
    1. Choose $u$ and $dif v$
    2. Find $dif u$ and $v$
    3. Apply the formula
  ],

  [
    *Matrix Multiplication*
    $ C_(i j) = sum_(k=1)^n A_(i k) dot B_(k j) $

    Example:
    $ mat(1, 2; 3, 4) mat(5, 6; 7, 8) = mat(19, 22; 43, 50) $
  ],

  [
    *Taylor Series*
    $ f(x) = sum_(n=0)^infinity frac(f^((n))(a), n!) (x-a)^n $

    Common series:
    - $e^x = sum_(n=0)^infinity frac(x^n, n!)$
    - $sin(x) = sum_(n=0)^infinity frac((-1)^n x^(2n+1), (2n+1)!)$
  ]
)
