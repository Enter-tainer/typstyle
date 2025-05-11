/// typstyle: wrap_text

Some pieces should be exclusion except $"inline equation"$ and `raw`:
- This is a list
$ "block equation" $
#figure([Figures])
#figure([Labeled figures]) <label>
That's all!

#let fig = (..) => []
#fig("image1")[Caption] <label1>
#fig("image2")[Caption] <label2>
