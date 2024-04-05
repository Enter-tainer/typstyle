# Changelog

## v0.11.11 - [2024-04-05]

- Fix set rules args are always spread into multiple lines. It now behaves like function call args.

For example, this code:
```typ
#set text(  font: body-font,
  lang: "zh",  region: "cn",
)
```

After formatting, it will become:
```typ
#set text(font: body-font, lang: "zh", region: "cn")
```

- Fix flavor detection for function call args. It now works correctly when the first space in the args contains a newline.

## v0.11.10 - [2024-04-02]

- Block math equations are no longer indented.
- We now support flavor detection for block equations. 

For example, this code:
```typ
$
  F(x) = integral_0^x f(t) dif t
$

$ F(x) = integral_0^x f(t) dif t
$

```

After formatting, it will become:
```typ
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
```typ
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
```typ
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
