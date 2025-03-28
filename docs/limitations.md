To ensure that source code remains valid, typstyle refrains from formatting in certain scenarios. The following cases outline situations where typstyle either does not apply formatting or applies only conservative changes.

## Overall

### `@typstyle off`

This directive explicitly disables formatting.

### Expressions with Comments

After months of work, we can now proudly format everything with comments!

We guarantee that in all supported cases the source will be formatted correctly and no comments will be lost.
If you find that a comment is lost or the formatting result is unsatisfactory due to comments, please submit a PR to present the issue.

### Spaces in Math

Math mode is highly sensitive to spacing, and users may play on content magics. Therefore, typstyle avoids changing spaces within math mode to ensure the evaluation result unchanged.

Additionally, typstyle will not convert spaces into line breaks (or vice versa) in math, as such changes can adversely affect the appearance of equations. We respect the user's intent regarding spaces and linebreaks.

## Special Cases

### Tables

Typstyle attempts to format tables into a neat, rectangular layout—but only when the table is simple enough. A table is considered "simple" if it meets all of the following conditions:

1. No comments.
2. No spread args.
3. No named args, or named args appears before all pos args.
4. No `{table,grid}.{vline,hline,cell}`.
5. `columns` is int or array.

Note that we can only recognize functions named `table` and `grid`, and `{table,grid}.{header,footer}` as args. Aliases or wrappers of `std.{table,grid}` are not supported.
