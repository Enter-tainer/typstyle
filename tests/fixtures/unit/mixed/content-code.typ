#let task(body, critical: false) = {
set text(red) if critical
[- #body]
}

由三个递增的数字组成，并用「点号」（`.`）分隔由三个递增的数字组成，并用「点号」（`.`）分隔由三个递增的数字组成，并用「点号」（`.`）分隔。

#task(critical: 
true)[Food today?]
#task(critical:
 false)[Work deadline]
