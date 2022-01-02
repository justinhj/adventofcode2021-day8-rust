# Day 8

```
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
```

Digits and the number of segments each has:

  0:6     1:2     2:5     3:5     4:4
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:5    6:6    7:3     8:7    9:6
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

Insights...

A set of wires in the input must be mapped to a digit 
Some can be deduced because there is a unique number of digits

6,5,5,5,6,6
2,3,4,7
Then three 6's and 5's

Digits with uniques are 1,7,4 and 8

Part one is to simply parse the input and count the number of elements with uniques.



