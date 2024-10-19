# Changelog

## v0.12.0 - [2024-10-19]

- Bump to typst v0.12.0
- Support new import syntax. Now long import can be broken into multiple lines.

Previous:

```typst
#import "test.typ": aaa, bbb as cccccccccc, ddd as eeeeeeeeeee, fff as g
```

Now:

```typst
#import "test.typ": (
  aaa,
  bbb as cccccccccc,
  ddd as eeeeeeeeeee,
  fff as g,
)
```

## v0.11.35 - [2024-10-07]

- Fix block comments drifting right if they have indentation. Now we strips all leading whitespaces in block comments.

## v0.11.34 - [2024-09-22]

- Fix a bug in the `completions` subcommand. https://github.com/Enter-tainer/typstyle/pull/131#issuecomment-2365456088

## v0.11.33 - [2024-09-22]

- feat: add command-line completions
```
Generate shell completions for the given shell to stdout

Usage: typstyle completions <SHELL>

Arguments:
  <SHELL>  The shell to generate completions for [possible values: bash, elvish, fish, powershell, zsh]
```

## v0.11.32 - [2024-08-19]

- Bug fix: Typstyle previously fails to correctly format inline triple backtick code block without a lang tag or an empty inline triple backtick code block with only a lang tag. Now it is fixed.
```typst
#text(``` test ```)
#text(```test ```)
```

Previously, it will be formatted as:
```typst
#text(```test  ```)
#text(```test  ```)
```

Now it is fixed.

## v0.11.31 - [2024-08-08]

- Bug fix: Typstyle previously removes necessary leading colon in dict. Now it is fixed.
```typst
#{
  let a = (a: 1)
  let b = (b: 2)
  (: ..a, ..b) // previously it will be formatted as (..a, ..b)
}
```

## v0.11.30 - [2024-07-14]

- Bug fix: previously when a destructing pattern has extra parentheses, typstyle will completely remove everything inside the parentheses. Now it is fixed.
- Typstyle now collapses extra parentheses in expression. 

## v0.11.29 - [2024-07-13]

- typstyle cli now can be installed from `cargo-binstall`
- typstyle now recognize dot chains and keep them aligned on multiple lines when possible.

Previously, typstyle's format result looks like this:
```typst
#{
  let (title, _) = query(heading.where(level: 1)).map(e => (
    e.body,
    e.location().page(),
  )).rev().find(((_, v)) => v <= page)
}

```

Now it will be formatted as:
```typst
#{
  let (title, _) = query(heading.where(level: 1))
    .map(e => (e.body, e.location().page()))
    .rev()
    .find(((_, v)) => v <= page)
}
```

- Minor adjustment for closure body formatting. 

## v0.11.28 - [2024-06-25]

- typstyle cli now has a `--check` flag to check if the input is formatted. If it's not formatted, it will return a non-zero exit code.
- Allow disabling git info collection in build time.

## v0.11.27 - [2024-06-20]

- Fix #97. Typstyle previously add an extra newline for `table` and `grid` when there is no positional argument and there are extra arguments. Now it doesn't add an extra newline.
- Typstyle cli now returns non-zero exit code when there are formatting errors.

## v0.11.26 - [2024-06-13]

- Typstyle now keeps newlines in function call args. Multiple newlines in function call args are common in fletcher diagrams. Before this release, typstyle removes all extra newlines in function call args. Now it keeps them as they are.

<details><summary>Example</summary>

```typst
#set text(10pt)
#diagram(
  node-stroke: .1em,
  node-fill: gradient.radial(blue.lighten(80%), blue, center: (30%, 20%), radius: 80%),
  spacing: 4em,

  node((0,0), `reading`, radius: 2em),
  node((1,0), `eof`, radius: 2em),
  node((2,0), `closed`, radius: 2em, extrude: (-2.5, 0)),

  edge((-1,0), "r", "-|>", `open(path)`, label-pos: 0, label-side: center),
  edge(`read()`, "-|>"),
  edge(`close()`, "-|>"),
  edge((0,0), (0,0), `read()`, "--|>", bend: 130deg),
  edge((0,0), (2,0), `close()`, "-|>", bend: -40deg),
)

```

After formatting, it will become this. Notice the extra newlines are kept.
```typst
#set text(10pt)
#diagram(
  node-stroke: .1em,
  node-fill: gradient.radial(
    blue.lighten(80%),
    blue,
    center: (30%, 20%),
    radius: 80%,
  ),
  spacing: 4em,

  node((0, 0), `reading`, radius: 2em),
  node((1, 0), `eof`, radius: 2em),
  node((2, 0), `closed`, radius: 2em, extrude: (-2.5, 0)),

  edge((-1, 0), "r", "-|>", `open(path)`, label-pos: 0, label-side: center),
  edge(`read()`, "-|>"),
  edge(`close()`, "-|>"),
  edge((0, 0), (0, 0), `read()`, "--|>", bend: 130deg),
  edge((0, 0), (2, 0), `close()`, "-|>", bend: -40deg),
)
```
</details>


- For tables, if typstyle is [unable to format it in a column-aware way](https://enter-tainer.github.io/typstyle/limitations/#table), it will now format each arg, but do not reflow them. That is, the relative position of each arg is kept. If you put something in a single line, it will stay in a single line. Newlines are also kept.

<details><summary>Example</summary>

```typst
#table(
  columns: 4 * (1fr,),
  
  [a], [b], [c], [d],
  fill: (_, y) => if y == 0 { black },
  table.cell(rowspan: 2)[aa], table.cell(colspan: 2)[bc], [d],
  [b], table.cell(colspan: 2)[cd],
)
```

After formatting, it will become this. Notice the relative position of each arg is kept.
```typst
#table(
  columns: 4 * (1fr,),

  [a], [b], [c], [d],
  fill: (_, y) => if y == 0 {
    black
  },
  table.cell(rowspan: 2)[aa], table.cell(colspan: 2)[bc], [d],
  [b], table.cell(colspan: 2)[cd],
)
```

</details>

## v0.11.25 - [2024-06-09]

- Typstyle now keeps extra newlines in markup mode. Multiple newlines are sometimes used to separate different sections in a document or act as a paragraph placeholder. Typstyle now keeps them as they are.

```typst
== Unfinished Title



=== Section 1



=== Section 2
```

Previously, it will be formatted as:
```typst
== Unfinished Title

== Section 1

== Section 2
```

Now it is kept as it is.

## v0.11.24 - [2024-05-27]

- Now typstyle can format table with `table.header` and `table.footer` attributes. The header and footer will be put in a single line if possible.
  For what it cannot do, see https://github.com/Enter-tainer/typstyle/issues/59#issuecomment-2132252514.

```typst
#table(
  columns: 3,
  table.header(
    [Substance],
    [Subcritical °C],
    [Supercritical °C],
    repeat: true,
  ),
  [Hydrochloric Acid],
  [12.0],
      [92.1],
  [Sodium Myreth Sulfate],
  [16.6],
  [104],
        [Potassium Hydroxide],
  [24.7],
  [114.514],
)
```

After formatting, it will become:
```typst
#table(
  columns: 3,
  table.header(
    [Substance],
    [Subcritical °C],
    [Supercritical °C],
    repeat: true,
  ),

  [Hydrochloric Acid], [12.0], [92.1],
  [Sodium Myreth Sulfate], [16.6], [104],
  [Potassium Hydroxide], [24.7], [114.514],
)
```

## v0.11.23 - [2024-05-25]

- Enhance table formatting. When a table row cannot fit in a single line, each cell will be put in a single line.

For example, this code:
```typst
#figure(
  grid(
    columns: (auto, auto),
    rows: (auto, auto),
    gutter: 0em,
    [ #image("assets/1.png", width: 59%) ], [ #image("assets/2.png",width: 55%) ],

  ),
  caption: [],
)
```

After formatting, it will become:
```typst
#figure(
  grid(
    columns: (auto, auto),
    rows: (auto, auto),
    gutter: 0em,
    [ #image("assets/1.png", width: 59%) ],
    [ #image("assets/2.png", width: 55%) ],
  ),
  caption: [],
)
```
## v0.11.22 - [2024-05-20]

- Typstyle now can format table and grid in a "column-aware" way. It now recognizes basic patterns and column numbers, and put a single row in a single line if possible.

For example, this code:
```typst
#table(
  columns: 3,
    [Substance],
    [Subcritical °C],
    [Supercritical °C],
  
  [Hydrochloric Acid],
  [12.0], [92.1],
  [Sodium Myreth Sulfate],
  [16.6], [104],
  [Potassium Hydroxide],
  [24.7],
  [114.514]
)
```

After formatting, it will become:
```typst
#table(
  columns: 3,
  [Substance], [Subcritical °C], [Supercritical °C],
  [Hydrochloric Acid], [12.0], [92.1],
  [Sodium Myreth Sulfate], [16.6], [104],
  [Potassium Hydroxide], [24.7], [114.514],
)
```

## v0.11.21 - [2024-05-16]

Bump to typst v0.11.1

## v0.11.20 - [2024-05-15]

Typstyle cli now include a `format-all` subcommand to format all files in a directory in-place.

```sh
typstyle format-all dir
# or omit the dir to format the current directory
typstyle format-all
```

## v0.11.19 - [2024-05-11]

- Typstyle now indent block math equations.

For example, this code:
```typst
$
E = mc^2
$
```

Now it will be formatted as:
```typst
$
  E = mc^2
$
```

## v0.11.18 - [2024-05-09]

- Typstyle now can keep line comments attached to the end of the line when formatting code blocks.

For example, this code:
```typst
#{
  let c = 0 // my comment
}
```

Previously, the comment will be moved to the next line after formatting. Now it's attached to the end of the line.

```typst
#{
  let c = 0 // my comment
}
```

## v0.11.17 - [2024-05-03]

- Fix typstyle cli not stripping trailing spaces.

## v0.11.16 - [2024-05-01]

- Fix comment loss in closure definition

## v0.11.15 - [2024-04-22]

- Fix comment loss in destruction and set rules

Previously for this code, the comment will be removed after formatting. Now it's kept.

```typst
#let (
// abc
a, b, c,
) = (1, 2, 3)


#set text(
  size: 10pt,
  fallback: false,
  // lang: "de",
)
```

## v0.11.14 - [2024-04-19]

- API Change: allow takes a `typst::Source` as input to avoid re-parsing

## v0.11.13 - [2024-04-12]

- (#49) typstyle cli now support multiple input files. If multiple files are provided, they will be processed in order.
  This is especially useful when you want to format multiple files inplace with a single command.

  ```bash
  typstyle -i **/*.typ
  ```

## v0.11.12 - [2024-04-09]

- Improve performance when formatting nested structures.

Previously it takes infinite time to format this code:
```typst
#let f(..arg) = arg

#f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(1,2,3))))))))))))))))))))))
```

Now it is done in instant.

## v0.11.11 - [2024-04-05]

- Fix set rules args are always spread into multiple lines. It now behaves like function call args.

For example, this code:
```typst
#set text(  font: body-font,
  lang: "zh",  region: "cn",
)
```

After formatting, it will become:
```typst
#set text(font: body-font, lang: "zh", region: "cn")
```

- Fix flavor detection for function call args. It now works correctly when the first space in the args contains a newline.

## v0.11.10 - [2024-04-02]

- Block math equations are no longer indented.
- We now support flavor detection for block equations. 

For example, this code:
```typst
$
  F(x) = integral_0^x f(t) dif t
$

$ F(x) = integral_0^x f(t) dif t
$

```

After formatting, it will become:
```typst
$
F(x) = integral_0^x f(t) dif t
$

$ F(x) = integral_0^x f(t) dif t $
```

## v0.11.9 - [2024-04-01]

- Trailing spaces are now trimmed.
- Spread args/array/dict into multiple lines if the first space in it contains a newline. This enables flexible control over the formatting of spread args.
  This is called flavor detection.

For example, this code:
```typst
#let my-f(arg1, arg2,
  args: none) = {
  arg1 + arg2
}

#let my-f(arg1,
 arg2, args: none) = {
  arg1 + arg2
}

```

After formatting, it will become:
```typst
#let my-f(arg1, arg2, args: none) = {
  arg1 + arg2
}

#let my-f(
  arg1,
  arg2,
  args: none,
) = {
  arg1 + arg2
}
```

## v0.11.8 - [2024-03-31]

- Fix multiline string/single-backtick-raw-block being wrongly formatted
- Fix missing trailing comma single element array destruct
- Fix `#` is missing in some math environments

## v0.11.7 - [2024-03-30]

- Fix import rename being wrongly formatted

## v0.11.6 - [2024-03-29]

- Fix raw block that starts/ends with backtick is wrongly formatted
- Add version string in `--version` output

## v0.11.5 - [2024-03-28]

- Fix long import item being spread across multiple lines
- Fix bad formatting of destruct items
- Enable formatting when line comment presents in code block

## v0.11.4 - [2024-03-27]

- Put `clap` and `wasm-bindgen` under feature flags to reduce binary size when use as a library

## v0.11.3 - [2024-03-26]

- Nothing new. Just testing ci auto-release

## v0.11.2 - [2024-03-24]

- Fix math attach and function call mis-formatting

## v0.11.1 - [2024-03-21]

- Read from stdin when no arguments are provided

## v0.11.0 - [2024-03-18]

- Initial release
