#show raw.where(block: false): it => if it.text.starts-with("<") and it.text.ends-with(">") {
    set text(1.2em)
    doc-style.show-type(it.text.slice(1, -1))
  } else { 
    it 
  }
