#import "../book.typ": *

#show: book-page.with(title: "CLI Usage")

= CLI Usage

#raw(read("../assets/generated/cli-help.txt"), block: true)

== Basic Usage

```bash
# Format multiple files
typstyle -i chapter1.typ chapter2.typ appendix.typ

# Format entire project
typstyle -i .

# Format with specific configuration
typstyle -l 100 -t 4 --wrap-text -i src/
```

== Arguments

=== Input Files

```bash
# Format from stdin (default)
cat file.typ | typstyle

# Format specific files
typstyle file1.typ file2.typ

# Format directories (recursively)
typstyle src/ docs/
```

== Main Options

=== Format Control

==== In-Place Formatting

```bash
# Modify files directly
typstyle -i file.typ
typstyle --inplace file.typ
```

==== Check Mode

```bash
# Exit with non-zero if formatting needed
typstyle --check src/
```

=== Format Configuration

==== Line Width

```bash
# Set maximum line width (default: 80)
typstyle -l 100 file.typ
typstyle --line-width 100 file.typ
```

==== Indentation

```bash
# Set indentation width (default: 2)
typstyle -t 4 file.typ
typstyle --indent-width 4 file.typ
```

==== Text Wrapping (Experimental)

```bash
# Wrap text in markup to fit line width
typstyle --wrap-text file.typ
```

== Debug Options

=== AST Output

```bash
# Print the Abstract Syntax Tree
typstyle -a file.typ
typstyle --ast file.typ
```

=== Pretty Document Output

```bash
# Print the internal pretty document representation
typstyle -p file.typ
typstyle --pretty-doc file.typ
```

== Logging Options

=== Verbose Logging

```bash
# Enable verbose logging
typstyle -v file.typ
typstyle --verbose file.typ
```

=== Quiet Mode

```bash
# Print only diagnostics
typstyle -q file.typ
typstyle --quiet file.typ
```
