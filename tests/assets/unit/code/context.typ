#set text(lang: "de")
#context [
  #set text(lang: "fr")
  #text.lang \
  #context text.lang
]

#context   {  text.size   }

#context  {
   text.size }

#context { text.size 
  }

// issues/129
#let foo() = context {
     5
}

#let foo() = context {  5
}

#let foo() = context {  
     5 }

#let foo() = context {  5  }

#let foo = context {
     5
}

#let foo = context {  5
}

#let foo = context {  
     5 }

#let foo = context {  5  }
