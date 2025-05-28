#import "../book.typ": *

#show: book-page.with(title: "Formatting Features")

= Formatting Features

typstyle follows a consistent set of formatting rules to ensure your Typst code is readable, maintainable, and follows established conventions. This page documents the default formatting styles applied by typstyle.

== Line Width and Indentation

- *Default line width*: 80 characters (configurable with `--line-width`)
- *Default indentation*: 2 spaces per level (configurable with `--indent-width`)
- *No tabs*: typstyle always uses spaces for indentation

== Function Calls and Arguments

=== Function Call Arguments

Arguments in function calls are intelligently spaced and aligned. typstyle can handle various argument patterns including trailing commas and complex nesting:

```typst
#figure(caption: [A very long caption that exceeds the line width],placement:top,supplement:[Figure])

#text(weight:"bold")[Bold text]

#link("http://example.com")[test]

#table(    )[123][456]
```

=== Flavor Detection

typstyle uses "flavor detection" to determine formatting style. If the first space in arguments contains a newline, arguments are spread across multiple lines:

```typst
#let my-f(arg1, arg2,
  args: none) = {
  arg1 + arg2
}
```

=== Comments in Function Calls

Function calls with comments are properly formatted:

```typst
#{
  let x = f(
  cetz.draw.super-long-name.line(
    start: (0, 0),
    end: (1, 1),      // note
  ) // my comment
)
}
```

=== Combinable Expressions

When the only argument is "combinable", it stays on the same line:

```typst
#figure(
  fletcher.diagram(
    node-outset: .5em,
    node((+1, 0), [variable], radius: 3em),
  )
)
```

=== Last Argument Special Case

When the last argument is blocky (like closures or content blocks) or the only argument is combinable, typstyle keeps it on the same line as the function call:

```typst
#set page(
  margin: 0.5in,
  footer: context {
    if counter(page).display() == "2" {
      [test]
    } else {
      []
    }
  }
)

#figure(
  fletcher.diagram(
    node-outset: .5em,
    node-stroke: .075em,

    node(
      (+1,0,),
      [variable],
      radius: 3em,
    ), // test
  ))
```

== Code Blocks

=== Single Statement Blocks

Single-statement blocks can stay inline if they fit:

```typst
// Preferred for short statements
#let x = if true { 1 } else { 2 }
```

=== Newline Management

typstyle strips excessive newlines in code blocks:

```typst
#{


  let x = 1

  let y = 2


}
```

== Math Equations

=== Alignment

typstyle aligns `&` symbols in math equations, even with multiline cells. Non-block equations are never aligned:

```typst
$1/2x + y &= 3 \ y &= 3 - 1/2x$

$
F_n&=sum_(i=1)^n i^2&n > 0 \
a&<b+1&forall b < 1
$

$
a&=cases(
x + y, "if condition A",
z + w, "if condition B"
) \
b&=matrix(
1, 2;
3, 4
) \
c&=sum_(i=1)^n x_i
$
```

=== Spacing Rules

- Spaces are preserved around fractions when they exist
- No padding is added to the last cell in math alignments
- Backslashes are preserved
- Inline equations are never aligned or padded
- Spaces between variables and underscores are preserved: `$ #mysum _(i=0) $`

=== Comments in Math

typstyle can format math equations with comments while preserving their meaning:

```typst
$frac(// numerator
x, /* denominator */ y)$

$mat(1, /* row 1 */ 2; 3, // row 2
4)$

$sum_(i=1 /* start */ )^(n // end
) x_i$
```

=== Block vs Inline Equations

typstyle uses flavor detection for equations. Block equations with initial newlines are formatted with proper indentation:

```typst
$
  F(x) = integral_0^x f(t) dif t
$

$ F(x) = integral_0^x f(t) dif t
$
```

== Binary Expressions and Operators

=== Operator Chains

Binary expressions are formatted as operator chains with proper breaking and alignment:

```typst
/// typstyle: max_width=40
#let _is_block(e,fn)=fn==heading or (fn==math.equation and e.block) or (fn==raw and e.has("block") and e.block) or fn==figure or fn==block or fn==list.item or fn==enum.item or fn==table or fn==grid or fn==align or (fn==quote and e.has("block") and e.block)
```

== Dot Chains

=== Smart Breaking

Dot chains are broken intelligently based on complexity and function calls:

```typst
// Simple chains stay inline
#node.pos.xyz

// Complex chains with multiple calls break
#{let hlines_below_header = first-row-group-long-long.row_group-long-long-long-long.hlines-long-long-long-long}

#{
  let (title, _) = query(heading.where(level: 1)).map(e => (e.body, e.location().page())).rev().find(((_, v)) => v <= page)
}

#{padding.pairs().map((k, x) => (k, x * 1.5)).to-dict()}
```

== Tables and Grids

=== Formatting Behavior

- General Rules
  - `header`, `footer`, and line comments (`//`) always occupy their own lines.
  - Block comments disable table formatting entirely.
  - Blank lines are preserved and prevent reflow across them.
- Header & Footer
  - Both follow the table’s defined column layout.
- Cell Reflow
  - Reflow applies only when *no special cells* are present.
    Special cells include:
    - `cell`
    - `hline`
    - `vline`
    - Spread args (`..`)
  - If no special cells exist, typstyle reflows all cells to fit the columns.
  - Otherwise, the original grid structure is preserved.

=== Automatic Formatting

typstyle formats tables and grids in a "column-aware" way, recognizing basic patterns and column numbers. Single rows are kept on single lines when possible:

```typst
#table(
  columns: (auto, 1fr,) + (auto,),
  [Ethanol], [78.2], [241.0],   [Methanol],
   [64.7], [239.5],
  [Propanol], [97.4], [263.7], table.footer(
  repeat: true,     [Alcohols], [Average], [>240°C]
  )
)
```

When a table row cannot fit on a single line, each cell is placed on its own line:

```typst
#figure(
  grid(
    columns: (auto,auto),
    rows: (auto,auto),
    gutter: 0em,
    [#image("assets/1.png",width: 59%)],[#image("assets/2.png", width: 55%)],
    [#image("assets/3.png",width: 1fr)],[#image("assets/4.png", width: 2fr)],

  ),
  caption: [],
)
```

=== Complex Table Support

- Headers and footers are formatted as tables
- Special elements (`cell`, `hline`, `vline`) are recognized without prefixes
- Column count calculation handles complex expressions like `((1fr,) * 2 + 2 * (auto,)) * 3`
- Headers, footers, and table cells with rowspan/colspan are properly handled

```typst
#table(
  columns: 7,
  rows: (2.5em,) * 3,
  align: horizon + center,
  table.cell(rowspan: 2)[],table.cell(colspan: 2)[Header 1],table.cell(colspan: 2)[Header 2],table.cell(colspan: 2)[Header 3],
  ..([Data],[Info]) * 3,
  $-2$,$20degree 9'$,$577.48$,$20degree 17'$,$581.14$,$15degree 5'$,$436.24$,
  $-1$,$9degree 54'$,$576.44$,$9degree 57'$,$579.32$,$7degree 29'$,$436.66$,
)
```

== Import Statements

=== Soft Wrapping

Import statements use soft wrapping for long item lists, keeping them compact yet readable:

```typst
#import "module.typ": very,long,list,of,imported,items,that,exceeds,line,width,and_,continues,wrapping

#import "@preview/fletcher:0.5.7" as fletcher:diagram,node,edge
```

=== Alphabetical Sorting

When enabled with `--reorder-import-items` (default), import items are sorted alphabetically:

```typst
#import "module.typ": zebra,alpha,beta,gamma
```

== Content Blocks

=== Intelligent Breaking

Content blocks break into multiple lines when they have leading or trailing spaces:

```typst
#{
  let res = if true [ The Result is definitely true. ]else[ false. ]
}
```

== Lists and Enumerations

=== Indentation Preservation

List indentation is preserved and corrected:

```typst
 -   Fruit
      - Apple
  -     Banana
- Vegetable
      -    Carrot
      - Tomato
```

=== Content Block Lists

Lists in content blocks are properly surrounded with linebreaks:

```typst
#{
  [- single]
  [- indented
  - less
  ]
  [- indented
   - same
  - then less
   - then same
  ]
  [- indented
    - more
   - then same
  - then less
  ]
}
```

== Comments

=== Inline Comments

Inline comments are preserved and properly positioned:

```typst
#let conf(
  title: none,//comments
  authors: (),
  abstract: [],
  lang: "zh",// language
  doctype: "book",//comments
  doc,// my docs
) = {
  doc
}

#{
  let c = 0// my comment
}
```

=== Block Comments

Block comments are aligned and formatted:

```typst
#{
  let x = 1   /* Attached block comment
      that spans
 multiple lines
  */

  /* Block comment
      that spans
 multiple lines
  */

  /* Block comment with leading stars
    *  that
        *  spans
 *  multiple
    *  lines
  */
}
```

== Text Wrapping

When `--wrap-text` is enabled:

```typst
/// typstyle: wrap_text, max_width=30
Let's say you have a long text that needs to be wrapped in the markup. This is a very long sentence.
```

- Line breaks after backslashes in markup are preserved
- Labels are properly handled during wrapping
- Intentional spacing is maintained

== Parentheses Management

=== Unnecessary Parentheses Removal

typstyle removes unnecessary parentheses around:
- Literals
- Arrays
- Dictionaries
- Destructuring patterns
- Blocks
- Patterns

```typst
#let  (( (( ((a)),))) ) =((( ( (1)),)) )
#let  (((( (( (a)),)) ))) =((((( (1)),)) ))

#let a = ((b:((c : ((3))))))
#let a = ({(true)})
#let a = (([()]))
```

Parentheses around identifiers are kept for safety.

```typst
#let name = "naming";
#let a = (name: 1)
#let b = ((name): 1)
#let c = (((name)): 1)
#let d = ("name": 1)
#let e = (("name"): 1)
```

== File Endings

typstyle always ensures files end with a newline character, maintaining Unix conventions.

== Escape Hatch

Use `// @typstyle off` to disable formatting for specific regions:

```typst
// @typstyle off
#let intentionally_bad_format    =    "preserved";
// @typstyle on
#let properly_formatted="cleaned up"
```

For details, please see #cross-link("/pages/escape-hatch.typ")[Escape Hatch].
