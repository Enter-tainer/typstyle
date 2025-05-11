/// typstyle: wrap_text

This is a paragraph with // an inline comment that shouldn't break
the flow of text or create a new line. The formatter should handle
// multiple comments spread across
several lines while maintaining proper *formatting* and `code elements`.

Here's text with /* block-style comments */ mixed with *strong text*
and $"math" "equations"$ to ensure /* multi-line
   block comments */ don't interfere with text flow or // inline elements.

Testing comments near special characters:
a + b // comment after plus
x - y /* comment after minus */
term / definition // comment after slash
heading = title // comment after equals

A complex case mixing everything:
*bold text* // comment after strong
`code block` /* block comment */ with $"math"$ // final comment
and #text(red)[colored text] // with formatting
