
#set page(margin: 1in)
#set text(size: 10pt)

= Code Formatting and Comment Handling

== Function Definitions with Extensive Comments

// Main configuration function with detailed documentation
#let configure-document(
  // Document metadata
  title: "Untitled", // The document title
  authors: (), /* List of authors - can be strings or
                  dictionaries with name and affiliation */
  date: datetime.today(), // Publication date

  // Layout settings
  page-size: "a4", // Paper size: a4, letter, etc.
  margin: 1in, /* Page margins - can be single value
                  or dictionary with x/y values */
  columns: 1, // Number of columns (1-3)

  // Typography options
  font: "Linux Libertine", // Main body font
  heading-font: none, /* Heading font, defaults to body font
                        if not specified */
  math-font: "New Computer Modern Math", // Math font

  // Advanced options
  line-spacing: 0.65em, // Space between lines
  paragraph-spacing: 0.65em, // Space between paragraphs
  show-outline: true, /* Whether to include table of contents
                        at the beginning */

  // Content processing
  content // The main document content
) = {
  // Set up page configuration
  set page(
    paper: page-size,
    margin: margin,
    columns: columns,
    header: locate(loc => {
      // Only show header after first page
      if counter(page).at(loc).first() > 1 [
        #align(center)[#title] // Centered title in header
      ]
    }),
    footer: locate(loc => [
      #align(center)[
        // Page numbering with total pages
        Page #counter(page).display() of #counter(page).final().first()
      ]
    ])
  )

  // Typography settings
  set text(
    font: font,
    size: 11pt,
    lang: "en" // Language for hyphenation
  )

  /* Heading configuration with automatic numbering
     and consistent spacing */
  show heading: it => {
    // Different styling for different heading levels
    let size = if it.level == 1 {
      16pt // Main headings
    } else if it.level == 2 {
      14pt // Section headings
    } else {
      12pt // Subsection headings
    }

    // Consistent spacing around headings
    v(1em) + text(
      size: size,
      weight: "bold",
      font: if heading-font != none { heading-font } else { font }
    )[#it.body] + v(0.5em)
  }

  // Math configuration
  set math.equation(
    numbering: "(1)", // Equation numbering format
    supplement: [Eq.] // Reference prefix
  )

  // Title page generation
  if title != "Untitled" or authors.len() > 0 {
    align(center)[
      // Main title
      #text(size: 20pt, weight: "bold")[#title]

      #v(1em)

      // Authors list with proper formatting
      #if authors.len() > 0 {
        let author-list = authors.map(author => {
          if type(author) == str {
            author // Simple string author
          } else {
            // Complex author with affiliation
            [#author.name#if "affiliation" in author [
              \ #text(size: 9pt, style: "italic")[#author.affiliation]
            ]]
          }
        })

        // Join authors with appropriate separators
        if author-list.len() == 1 {
          author-list.first()
        } else if author-list.len() == 2 {
          author-list.join([ and ])
        } else {
          author-list.slice(0, -1).join([, ]) + [, and ] + author-list.last()
        }
      }

      #v(0.5em)

      // Date formatting
      #text(size: 10pt)[
        #date.display("[month repr:long] [day], [year]")
      ]
    ]

    pagebreak() // Start content on new page
  }

  // Table of contents
  if show-outline {
    outline(
      title: "Contents", // Customizable title
      indent: true, // Indent subsections
      depth: 3 // Maximum heading level to include
    )
    pagebreak()
  }

  // Main content with proper spacing
  set par(
    justify: true, // Justified text
    leading: line-spacing,
    spacing: paragraph-spacing
  )

  content // Insert the actual document content
}

== Complex Code Blocks with Comments

#{
  // Data processing pipeline with extensive commenting
  let raw-data = (
    (id: 1, value: 42.5, category: "A"), /* First data point
                                            with baseline measurements */
    (id: 2, value: 38.1, category: "B"), // Second measurement
    (id: 3, value: 51.3, category: "A"), /* Third point - note the
                                            higher value in category A */
    (id: 4, value: 29.7, category: "C"), // Outlier in new category
    (id: 5, value: 45.8, category: "B")  /* Final measurement for
                                            statistical significance */
  )

  /* Statistical analysis function
     Calculates mean, median, and standard deviation
     for numeric data arrays */
  let analyze-data(data, value-key: "value") = {
    // Extract values for analysis
    let values = data.map(item => item.at(value-key))

    // Calculate mean
    let mean = values.fold(0, (sum, val) => sum + val) / values.len()

    // Calculate median (requires sorting)
    let sorted = values.sorted()
    let median = if calc.rem(sorted.len(), 2) == 0 {
      // Even number of elements - average middle two
      (sorted.at(int(sorted.len() / 2 - 1)) + sorted.at(int(sorted.len() / 2))) / 2
    } else {
      // Odd number of elements - take middle
      sorted.at(int(sorted.len() / 2))
    }

    // Calculate standard deviation
    let variance = values.fold(0, (sum, val) => {
      sum + calc.pow(val - mean, 2)
    }) / values.len()
    let std-dev = calc.sqrt(variance)

    // Return results as dictionary
    (
      mean: calc.round(mean, digits: 2),
      median: calc.round(median, digits: 2),
      std-dev: calc.round(std-dev, digits: 2),
      min: values.fold(values.first(), calc.min),
      max: values.fold(values.first(), calc.max),
      count: values.len()
    )
  }

  // Group data by category for comparative analysis
  let group-by-category(data) = {
    let groups = (:) // Initialize empty dictionary

    for item in data {
      let cat = item.category
      // Create new group if it doesn't exist
      if cat not in groups {
        groups.insert(cat, ())
      }
      // Add item to appropriate group
      groups.at(cat).push(item)
    }

    groups // Return grouped data
  }

  // @typstyle off - preserve custom formatting for demonstration
  let   intentionally_spaced     =     "This formatting is preserved"
  let properly_formatted = "This will be cleaned up by typstyle"

  // Process the data
  let grouped = group-by-category(raw-data)
  let overall-stats = analyze-data(raw-data)

  /* Generate summary report
     Creates formatted output for analysis results */
  let generate-report(stats, grouped-data) = [
    == Data Analysis Summary

    *Overall Statistics:*
    - Count: #stats.count measurements
    - Mean: #stats.mean
    - Median: #stats.median
    - Standard Deviation: #stats.std-dev
    - Range: #stats.min – #stats.max

    *Category Breakdown:*
    #for (category, items) in grouped-data [
      - *Category #category:* #items.len() items
        - Values: #items.map(item => str(item.value)).join(", ")
        - Category Mean: #calc.round(
          items.fold(0, (sum, item) => sum + item.value) / items.len(),
          digits: 2
        )
    ]
  ]

  // Display the report
  generate-report(overall-stats, grouped)
}

== Conditional Logic with Complex Comments

#let process-grades(students) = {
  students.map(student => {
    let scores = student.scores
    let average = scores.fold(0, (sum, score) => sum + score) / scores.len()

    // Grade assignment with detailed criteria
    let letter-grade = if average >= 97 {
      "A+" /* Exceptional performance - rare achievement
             indicates mastery beyond course requirements */
    } else if average >= 93 {
      "A"  // Excellent work - clear understanding
    } else if average >= 90 {
      "A-" /* Very good work with minor areas
             for improvement */
    } else if average >= 87 {
      "B+" // Good work - meets most expectations
    } else if average >= 83 {
      "B"  /* Satisfactory work - meets basic
             requirements with some gaps */
    } else if average >= 80 {
      "B-" // Below average but passing
    } else if average >= 77 {
      "C+" /* Minimal passing grade - significant
             improvement needed */
    } else if average >= 70 {
      "C"  // Barely passing - major concerns
    } else {
      "F"  /* Failing grade - does not meet
             minimum requirements */
    }

    // Generate performance feedback
    let feedback = if average >= 90 {
      "Excellent performance! Keep up the great work."
    } else if average >= 80 {
      "Good work with room for improvement in some areas."
    } else if average >= 70 {
      "Satisfactory performance. Consider additional study time."
    } else {
      "Performance below expectations. Please see instructor."
    }

    // Return enhanced student record
    (
      ..student, // Spread existing fields
      average: calc.round(average, digits: 2),
      grade: letter-grade,
      feedback: feedback,
      // Status determination with complex logic
      status: if average >= 70 and student.attendance >= 0.8 {
        "Passing" /* Both grade and attendance requirements met
                     - student is on track */
      } else if average >= 70 {
        "At Risk" // Grade OK but attendance issues
      } else if student.attendance >= 0.8 {
        "Academic Probation" /* Attendance good but failing grade
                               - needs academic support */
      } else {
        "Failing" /* Both grade and attendance below standards
                     - intervention required */
      }
    )
  })
}

// Example usage with sample data
#let sample-students = (
  (
    name: "Alice Johnson",
    id: "12345",
    scores: (95, 87, 92, 89, 94), // Consistent high performance
    attendance: 0.95 // Excellent attendance
  ),
  (
    name: "Bob Smith",
    id: "12346",
    scores: (78, 82, 75, 80, 77), /* Borderline performance
                                    needs improvement */
    attendance: 0.85 // Good attendance
  ),
  (
    name: "Carol Davis",
    id: "12347",
    scores: (65, 58, 72, 61, 69), // Struggling academically
    attendance: 0.65 /* Poor attendance contributing
                        to academic difficulties */
  )
)

// Process and display results
#let processed = process-grades(sample-students)

#table(
  columns: (auto, auto, auto, auto, auto),
  stroke: 0.5pt,
  [*Name*], [*Average*], [*Grade*], [*Status*], [*Attendance*],
  ..processed.map(s => (
    s.name,
    str(s.average) + "%",
    s.grade,
    s.status,
    str(calc.round(s.attendance * 100, digits: 1)) + "%"
  )).flatten()
)
