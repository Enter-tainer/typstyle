/// typstyle: wrap_text

#[
  Outer block with some text that should wrap naturally across
  multiple lines when it reaches the margin     #[
    Nested block with *bold* and
    _italic_ text mixed with #emph[emphasis marks] and some

    #[Deep nested content containing
      #text(blue)[colored elements] and even
      deeper #[
        Level 4 nesting with
        #smallcaps[special formatting] and
           #[
          Maximum nesting depth with
             #text(weight: "bold")[important content] that demonstrates proper
          indentation and text wrapping at all levels        ]
      ]
    ]
  ]
]

      #[
  Another test with mixed block types

  #quote[
    Quoted text inside a block with

    #[
      Nested content combining
          #box(width: 80%)[
        Width-constrained box with deeply

        #[
          nested formatting and
             #text(tracking: 1.5pt)[tracked text] for testing
        ]      ]
    ]
  ]
]

#block[
  Testing block directive with
     #[
    nested content and
       #text(font: "serif")[
      different font styles
         #[
        with multiple layers of
           #emph[emphasis] and formatting
      ]
    ]
  ]
]
