// Basic multiple spaces and formatting
Text with  two spaces between words.

    Text with   three   spaces   between   words.     // line comment

*Bold*  with  two  spaces and _italic_   with   three   spaces.

`Code`    with    four    spaces and #smallcaps[Small Caps]  mixed.

#strike[Strikethrough]   and   #super[superscript]  and  #sub[subscript]   combined.

// Mixed content with spaces
Link  to  #link("url")[example]    and    #text(red)[colored]   text   together.

Math with  spaces: $x  +  y  =  z$  and  inline math  $a   b   c$   surrounding.

// Spaces at boundaries and special characters
   Leading and trailing     /*  block comment   */    spaces

Text  with  "quotes"   and   'apostrophes'    and    symbols:   &   <   >   {}   []

Emoji  😀  with  spaces  🎉  and  Unicode:  α  β  γ   characters.

// Nested markup and line breaks
This is #emph[emphasized  text  with  spaces] and #box[boxed   content] together.

Nested #strong[*bold*  inside  strong] with spaces  \
  and  line  breaks  \
   across   multiple   lines.

// Lists, quotes, and structural elements
- Item  with  spaces and nested    item    together
+ Numbered   list   with   1. Deep    nested    content

#quote[
  Quoted  text  with  spaces
     and   multiple   lines
]

= Heading  with  spaces and == Subheading   combined

// Complex structures and code
#align(center)[
  Centered  text  with  spaces
] and #stack[
     Stacked  content
  with   preserved   spaces
]

#table(
  columns: 2,
  [Cell  with  spaces], [Another   cell   with   spaces],
  [More    content], [Even     more     spaced     content]
)

Here's `inline  code  with  spaces` and code blocks:

```
Code block
with  preserved  spaces
```

#show ref: repr
// References and function calls
See @reference  for  @ref1  @ref2   multiple   citations.

Call #math.vec("arg1",  "arg2")  and  nested #text(text[content  with  spaces])  functions.

// Mixed whitespace and internationalization
Text	with	tabs	and  spaces  mixed    together.

Français  avec  des  中文  文本  与  العربية  مع  international   spaces.

Text with non-breaking and  en-spaces  and  em-spaces  together.
