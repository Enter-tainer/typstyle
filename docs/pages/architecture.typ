#import "../book.typ": *

#show: book-page.with(title: "Architecture")

= High Level Overview

Typstyle is a code formatter, the input is a string of typst source code and the output is a string of formatted typst source code.

To format the code, there are certain main steps that are followed:

+ *Parsing*: The input code is parsed into an Abstract Syntax Tree (AST) using the `typst-syntax` package. If the input code is erroneous, the code will not be formatted and following steps will be skipped.
+ *Attach Attributes*: The AST is traversed and certain attributes are attached to the nodes. Like some nodes should be skipped from formatting, some nodes should be formatted in a special way, etc.
+ *Formatting*: The AST is traversed and the AST is transformed into a Wadler-style pretty-print-tree. This tree is then converted into a string of formatted code.
+ *Post Processing*: The formatted code is post-processed to remove any trailing whitespaces, etc.
+ *Output*: The formatted code is returned as the output.

The main work happens in step 2 and 3. We will discuss these steps in detail in the following sections.
