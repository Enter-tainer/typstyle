To ensure source code remains valid, typstyle will refrain from formatting in certain scenarios. Below is a list of cases where typstyle does not apply formatting.

## Overall

### `@typstyle off`

Why: This directive explicitly disables formatting.

### Markup Lines

Typstyle only formats code and does not alter markup lines. These lines are preserved as-is. Specifically, if a line contains text (`ast::Expr::Text`), the entire line will remain unformatted.

### Math Mode

Formatting in math mode is minimal and not well-implemented at this time.

### Expressions with Comments

When a block comment is present as a child in math mode, the entire node is skipped for formatting:

Why: This require special handling to bring better reading experience. Interspersing comments within them introduces additional complexity that is not yet resolved.

We guarantee that in all formatable cases, no comments should be lost.
If any comments are lost, please submit a PR to present the issue.

### Multiline raw with single backtick

Why: These strings are whitespace-dependent.

```typst
`a
  b`
is not
`a
    b`
```

### Nodes with `#` in Math Mode

If a child node contains `#` in math mode, typstyle will skip formatting the entire node.

Why: Hashes can appear anywhere, and handling them accurately is challenging.

```typst
$f(a+b, size: #1em)$
```

### Args in Math Mode

Why: Arguments in math mode behave differently, such as 2D arguments and trailing commas.

```typst
$mat(a,,;,b,;,,c)$
```

## Special Cases

### Table

Typstyle attempts to format tables into a rectangular shape, but only when the table is simple enough. A table is considered "simple" if it meets the following conditions:

1. No comments.
2. No spread args.
3. No named args, or named args appears before all pos args.
4. No `table/grid.vline/hline/cell`.
5. `columns` is int or array.
