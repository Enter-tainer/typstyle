#import "../book.typ": *

#show: book-page.with(title: "Escape Hatch")

= Escape Hatch

Sometimes you may want to preserve specific formatting that differs from typstyle's default style, or typstyle may not handle certain cases optimally. The escape hatch feature allows you to selectively disable formatting for particular code sections using special comments.

== When to Use the Escape Hatch

While typstyle handles most Typst code well, there are situations where manual formatting is preferable:

1. *Intentional alignment and structure* that improves readability
2. *Complex data structures* where compact formatting is clearer
3. *Edge cases* where typstyle's formatting doesn't match your intent
4. *Workarounds* for formatting bugs or limitations

== Basic Usage

Use ```typ // @typstyle off``` or ```typ /* @typstyle off */``` to disable the formatter on the next non-trivial syntax node:

```typst
// Normal formatting applies here
#let normal = func(arg1, arg2)

// @typstyle off
#let preserved   =   bad_formatting   ;   // This line keeps its formatting

// Normal formatting resumes
#let formatted = another_func(arg1, arg2)
```

== Automatic Fallback

Typstyle automatically preserves original formatting when it encounters issues:

- *Syntax errors*: Code with parsing errors is left unchanged
- *Complex edge cases*: Rare constructs that may downgrade formatting capabilities

In these cases, you don't need an escape hatch—typstyle handles it automatically.

== Scope and Behavior

=== Parbreak Penetration

The escape hatch comment must be placed directly before the code you want to preserve. It doesn't work across paragraph breaks:

```typst
// @typstyle off

#(1+2)  // This will be formatted normally because of the blank line above
```

=== Comment Penetration

The escape hatch does not penetrate through comments:

```typst
// @typstyle off
/* This comment prevents formatting of the following code */
#let formatted=func(arg1,arg2)  // This will be formatted normally
```

=== Specific Syntax Nodes

When used before `Code` or `Math`, the escape hatch applies only to the first non-trivial child:

```typst
#{
  // @typstyle off
  let preserved=bad_formatting; // Only this first statement preserves formatting
  let formatted=normal_formatting(); // This will be formatted normally
}

$
  // @typstyle off
  sin( x ) // Only the first math element preserves formatting
  cos( y ) // This part will be formatted normally
$
```


== Use Cases

Common scenarios where the escape hatch is useful:

=== Intentional Alignment

When you have carefully aligned data that improves readability:

```typst
// @typstyle off
#table(
  columns: 4,
  [Name]    , [Age], [Height], [Weight],
  [Alice]   , [25] , [5'6"]  , [130lb] ,
  [Bob]     , [30] , [6'1"]  , [180lb] ,
  [Charlie] , [35] , [5'9"]  , [165lb] ,
)
```

```typst
// @typstyle off
#let matrix = (
  ( 1,  0,  0,  1),
  ( 0,  1,  0,  1),
  ( 0,  0,  1,  1),
  (-1, -1, -1,  0),
)
```

```typst
// @typstyle off
#let config = (
  width:  100pt,  // Total width
  height:  50pt,  // Total height
  margin:  10pt,  // Edge spacing
)
```

=== Dense Data Structures

For compact data where spacing improves readability:

```typst
// @typstyle off
#let coordinates = (
  (0,0),(1,0),(2,0),(3,0),
  (0,1),(1,1),(2,1),(3,1),
  (0,2),(1,2),(2,2),(3,2)
)
```
