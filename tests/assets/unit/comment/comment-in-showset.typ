#show:none
#show"foo":"bar"
#show heading:strong
#show heading.where(level: 1):set text(red)
#set text(size: 1.8em)if true

#show/* 0 */  :  /* 1 */none
#show  /* 0 */ "foo"/* 1 */:/* 2 */ "bar"
#show/* 0 */heading/* 1 */: /* 2 */strong
#show /* 0 */heading.where(level: 1)  /* 1 */   : /* 2 */set/* 3 */ text(red)
#set/* 0 */ text(size: 1.8em)/* 1 */if /* 2 */    true
