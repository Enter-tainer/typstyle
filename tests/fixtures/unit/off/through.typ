/* @typstyle off */
// line comment
#(   114  )

/* @typstyle off */ /* block */ #(   999  )

/* @typstyle off */
#1
#let   aaa  =  123
#let   bbb  =  123

/* @typstyle off */


#(1   +  4)

#( 1 + /* @typstyle off */ (3    /    9)    * 6)

#[
  /* @typstyle off */

  #(1+2+3)
]

#{
/* @typstyle off */

  (1+2+3)

}

#{
  // @typstyle off
  let   _ =   1
    let   _   =   2
let  _   =   3
}

#{
  let _ = 0
  // @typstyle off
  let   _ =   1
  let  _   =   2
  let _     = 3
}
#[
  // @typstyle off
  #let   _  =   1
  #let   _    =   2
]

$
  // @typstyle off
  sin(   x  )
  cos(  x  )
$
$
  sin(x)
  // @typstyle off
  cos(   x  )
  tan(  x  )
$
