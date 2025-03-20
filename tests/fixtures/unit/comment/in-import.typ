#import"test.typ"
#import"test.typ":
#import"test.typ":*
#import"test.typ":a  .  b .c.  d,eee as fff
#import"@preview/fletcher:0.5.2"as fletcher:node  ,edge
#import"@preview/fletcher:0.5.2"as fletcher:(node,edge)
#import"@preview/fletcher:0.5.2"as fletcher:(
  node,edge

  )
#import"@preview/fletcher:0.5.2"as fletcher:(
  node,
      edge )

#import/* 0 */"test.typ"/* 1 */
#import/* 0 */"test.typ"/* 1 */:/* 2 */
#import/* 0 */"test.typ"/* 1 */: /* 2 */
#import/* 0 */"test.typ"/* 1 */:  /* 2 */
#import/* 0 */"test.typ"/* 1 */:/* 2 *//* 3 */*
#import/* 0 */"test.typ"/* 1 */:/* 2 */a./* 3 */b/* 4 */.c. /* 5 */d, /* 6 */  eee/* 7 */as /* 8 *//* 9 */   fff
#import/* 0 */"@preview/fletcher:0.4.0"/* 1 */as/* 2 */fletcher/* 3 */:/* 4 */ node  /* 5 */  ,   /* 6 */edge
#import"@preview/fletcher:0.5.2"as fletcher:/* 0 */(/* 1 */node/* 2 */,/* 3 */edge/* 4 *//* 5 */)
#import"@preview/fletcher:0.5.2"as fletcher:(/* 0 */
/* 1 */  node/* 2 */,/* 3 */edge/* 4 */
/* 5 */
  /* 6 */)
#import"@preview/fletcher:0.5.2"as fletcher:(/* 0 */
/* 1 */  node/* 2 */,
/* 3 */edge/* 4 */
/* 5 */
  /* 6 */)
#import "block-short.typ":/* 0 */ (
  // 1
  a)
#import "block-short.typ":/* 0 */ (
  // 1
  b
  )
#import "block-short.typ":/* 0 */ (   // 1
  c)