#import "../../book.typ": *

#show: book-page.with(title: "Code Formatting")

#show: render-examples

= Code Formatting

== Code Block Structure

=== Single-Statement Blocks

Single-statement blocks remain inline when they fit within the line width, unless they are too long or have multiline flavor:

```typst
/// typstyle: max_width=40
#let x = if true { 1 } else { 2 }
#let x = if true {
  1 } else { 2 }
#let x = if true {
  1 } else {
     2 }
#let x = if true { "111111111111" } else { "222222222222222222222222222222" }
```

=== Linebreak Management

typstyle strips excessive newlines in code blocks:

```typst
#{


  let x = 1

  let y = 2


}
```

== Content Blocks

When content blocks have leading or trailing spaces, they break into multiple lines for better readability:

```typst
#{
  let res = if true [ The Result is definitely true. ]else[ false. ]
}
```


== Parentheses Removal

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

Parentheses around identifiers are preserved for safety to avoid changing semantics.

```typst
#let name = "naming";
#let a = (name: 1)
#let b = ((name): 1)
#let c = (((name)): 1)
#let d = ("name": 1)
#let e = (("name"): 1)
```

== Function Calls and Arguments

=== Argument Formatting

Function call arguments are intelligently spaced and aligned. typstyle handles various argument patterns including trailing commas and complex nesting:

```typst
#figure(caption: [A very long caption that exceeds the line width],placement:top,supplement:[Figure])
```

=== Flavor Detection

typstyle uses "flavor detection" to determine formatting style. If the first space in arguments contains a newline, arguments are spread across multiple lines:

```typst
#let my-f(arg1,
 arg2,
  args: none) = {
}
#let my-f(arg1, arg2,
  args: none) = {
}
```

=== Combinable Arguments

typstyle applies special formatting when the *last* argument is "combinable" - meaning it can naturally flow as a single unit with the function call. Combinable expressions include:

- *Blocky expressions*: code blocks, conditionals, loops, context expressions, closures
- *Structured data*: arrays, dictionaries
- *Nested calls*: function calls, parenthesized expressions
- *Content blocks*: markup content in square brackets

When the last argument is combinable, typstyle tries to use a *compact layout*: initial arguments are placed on the first line, while the last combinable argument spans multiple lines without extra indentation (ignoring normal flavor detection).

However, when compact layout isn't possible (e.g., initial arguments can't be flattened due to comments or intentional line breaks), typstyle falls back to an *expanded layout*.

```typst
#f(   if true {    let x = 3  })
#f(if true {
    let x = 3
  })
#f(    1111,    22222,    if true {
      let x = 3
      let y = 4
    },
  )
#f(    1111,    if true {      let x = 3   },    22222, )
  #f(    1111,    if true {      let x = 3 ;  let y = 4   },    22222, )
  #f(
    context {
      1
    }
  )

#set page(
  margin: 0.5in,
 footer: context {
  if counter(page).display() == "2" {
    [test]
  } else {
    []
  }
})
```

```typst
// Content block as last argument
#f(xx: 1, 2, 3, yyy: [
  Multiple line
  content in array
])

// Code block with multiple initial args
#f("string", aaa: (1, 2), bbb : (x: 1, y: 2), {
  let x = 1
  let y = 2
  x + y
})
```

*Exception*: If the same structured data type (array/dict) appears in earlier arguments, the last argument breaks to maintain visual distinction:

```typst
#f(
  (x: 1, y: 2), (a: 3, b: 4), (m: 5, n: 6),)
#f((x: 1, y: 2), (a: 3, b: 4), (m: 5, n: 6),)
```

== Chainable Expressions

=== Binary Chains

Binary expressions are formatted as operator chains with proper breaking and alignment:

```typst
/// typstyle: max_width=40
#let _is_block(e,fn)=fn==heading or (fn==math.equation and e.block) or (fn==raw and e.has("block") and e.block) or fn==figure or fn==block or fn==list.item or fn==enum.item or fn==table or fn==grid or fn==align or (fn==quote and e.has("block") and e.block)
```

=== Dot Chains

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

== Import Statements

=== Soft Wrapping

Import statements use soft wrapping for long item lists, keeping them compact yet readable:

```typst
#import "module.typ": very,long,list,of,imported,items,that,exceeds,line,width,and_,continues,wrapping

#import "@preview/fletcher:0.5.7" as fletcher:diagram,node,edge
```

=== Item Ordering

When enabled with `--reorder-import-items` (default), import items are sorted alphabetically:

```typst
#import "module.typ": zebra,alpha,beta,gamma
```
