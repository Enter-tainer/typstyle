#let conf(
  title: none, //comments
  authors: (),
  abstract: [],
  lang: "zh",   // language
  doctype: "book",  //comments
  doc  // all comments will be kept by typstyle
)={doc}

#set heading( /* 1 */ numbering   /* 2 */:/* 3 */ (/* 4 */../* 5 */num/* 6 */) /* 1 */   =>      none)

#let f()/* 0 */=/* 1 */   ()=>  /* 2 */none
#let g(..)/* 0 */   =  /* 1 */  ()=>  /* 2 */none
#let h(..)/* 0 */ =/* 1 */ ()=>/* 2 */   { none}

#let f = /* 0 */ ()/* 1 */=>  /* 2 */none
#let g =/* 0 */(..)/* 1 */=>/* 2 */   none
#let h =/* 0 */(..)/* 1 */=>  /* 2 */   { none}
