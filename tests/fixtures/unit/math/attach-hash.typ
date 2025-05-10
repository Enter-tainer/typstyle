#let my_sum = text(red)[$sum$]
#let my_dict = (my_field: 666)

$  #my_sum _123  ^ 456 $
$  #my_sum'_123  ^ 456 $
$  #my_sum '_123  ^ '456 $
$  #my_sum' _123  ^ ' 456 $
$  #my_sum _ #my_sum  ^ #my_sum _ #my_sum ^ #my_sum $
$ #my_dict.my_field _ #my_dict.my_field ^ #my_dict.my_field $
$ eq.not _ eq.not ^ eq.not _ eq.not $
$ eq.not ^ eq.not _ eq.not $
$ #sym.eq.not _ #sym.eq.not ^ #sym.eq.not _ #sym.eq.not $
$ #sym.eq.not ^ #sym.eq.not _ #sym.eq.not $

$ #1_#2^#3_#4 $
$ #1 _#2 ^#3 _ #4 $
$ #1_ #2 ^ #3_ #4 $
