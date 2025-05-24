// Alignment with decorations
$ arrow(x x &= y) \
  circle(a &= b b) \
  hat(c &= d d d) \ $

// Complex nested breaks
$ mat(1,2,3; 4,5,6) & = mat(a,b; & c,d; \
    & e,f) \
    & = "result" $

// Multi-level alignment with breaks
$ cases(
  x & text("case 1") & = alpha \
    & & = beta, \
  y & text("case 2") & = gamma \
    & & = delta
) $

// Breaking inside fractions
$ frac(
    1 + 2 + 3 & + 4 + 5,
    6 + 7 & + 8 + 9
  ) & = "result" $

// Nested structures with varying alignments
$ cases(
    1 &= x &= y,
    2 &= z,
    3 &= w &= v &= u
) $

// Cases with alignment and breaks in each branch
$ cases(
    x &= alpha &= beta \
      &= gamma &= delta,
    y &= epsilon \
      &= zeta \
      &= eta,
    z &= theta &= iota \
      &= kappa &= lambda \
      &= mu
) $

// Cases with mixed content and breaks
$ cases(
    "Case 1:" &= sum x_n \
                &= product y_n,
    "Case 2: " &= integral_0^1 f(t) \
              &= g(x),
    "Case 3:  " &= cases(
                a &= b \
                  &= c,
                d &= e \
                  &= f
              )
) $

// Cases that may fail to property align
$ a + lr(size: #1cm,
b +& c mid(|) \
       mid(|) d +& e) $

$ a + lr(size: #1cm, b +& c mid(|) \
       mid(|) d +& e) $

$ a + lr(size: #1cm, (b +& c mid(|) \
       mid(|) d +& e)) $

$ a + lr(size: #1cm, (b +& c mid(|) \
       mid(|) d +& e)) + f $

$ a + lr(size: #1cm, (
  b +& c mid(|) \
       mid(|) d +& e)) + f $

$ a + lr(size: #1cm, (
  b +& c mid(|) \
       mid(|) d +& e  )) + f $

$
  a + lr(
    size: #1cm, (
             b + & c mid(|) \
      mid(|) d + & e        )
  ) + f
$

$
  sin(
             b
             + & c
             mid(|) \
      mid(|) d + & e
       * f        )
$
