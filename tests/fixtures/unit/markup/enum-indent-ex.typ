#{
  [/ single:]
  [/ indented:
  / less:
  ]
  [/ indented:
   / same:
  / then less:
   / then same:
  ]
  [/ indented:
    / more:
   / then same:
  / then less:
  ]
}
---
#{
  [/ indented:
    / less:
  ]
  [/ indented:
    / same:
    / then less:
      / then same:
  ]
  [/ indented:
      / more:
    / then same:
    / then less:
  ]
}