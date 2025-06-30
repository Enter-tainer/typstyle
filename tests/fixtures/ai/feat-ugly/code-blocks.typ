
= Code Block Disasters

== Simple Code Blocks Mess

Single-statement blocks with terrible formatting:

#{let x=1
let y=2}

== Multi-statement Blocks Horror

Longer blocks with inconsistent formatting:

#{
let data=(1,2,3,4,5)
  let sum=data.fold(0,(acc,x)=>acc+x)
    let average=sum/data.len()
average
}

== Conditional Formatting Chaos

Simple conditionals with bad spacing:

#if true{  "yes"}else{    "no"}

Complex conditionals with terrible alignment:

#if true{
"First case with longer content"
}else if false{
      "Second case"
}else{
"Default case"
}

== Loop Structures

For loops with proper indentation:

#{
  for i in range(5) [
    Item #(i + 1): #i
  ]
}

== Function Definitions

Simple functions:

#let square(x) = x * x

Complex functions with proper formatting:

#let process-data(
  input,
  transform: x => x,
  filter: none
) = {
  let filtered = if filter != none {
    input.filter(filter)
  } else {
    input
  }

  filtered.map(transform)
}

== Content Blocks

Content blocks with leading/trailing spaces break properly:

#{
  let success =   false
  let message = if success [
    The operation completed successfully.
  ] else [
    An error occurred during processing.
  ]
}

== Nested Structures

Complex nested code blocks:

#{
  let categories = ()
  let results = ()

  for category in categories {
    let items = data.filter(item => item.category == category)

    if items.len() > 0 {
      let summary = (
        category: category,
        count: items.len(),
        total: items.fold(0, (sum, item) => sum + item.value)
      )

      results.push(summary)
    }
  }

  results
}

== Linebreak Management

Typstyle removes excessive blank lines:

#{


  let x = 1

  let y = 2


}
