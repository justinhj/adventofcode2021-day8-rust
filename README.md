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

Digits with uniques are 1(2),7(3),4(4) and 8(7)

Part one is to simply parse the input and count the number of elements with uniques.

Next step, we want to fill out a map of new digit to original digit. 

For example:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc

 aaaa  
b    c 
b    c 
 dddd  
e    f 
e    f 
 gggg  

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

Here the original digit 0 is below and the new mapping is above.

Digits with uniques are 1(2),7(3),4(4) and 8(7)

Start with 1 (c,f)
We see that 1 (a,b) so either c is a and f is b or vice versa
So the solution could look like c -> (a,f), b -> (a,f)
Next 7 (a,c,f)
7 is (d,a,b) so d -> (a,c,f), a -> (a,c,f), b -> (a,f) NOTE we know that b is not c from above
Next 4 (b,c,d,f)
4 is (e,a,f,b) so e -> (b,c,d,f), a -> (c,f), f -> (b,c,d,f), b -> (f) NOTE THAT we know b now
so now we know b we can remove it from everywhere else
Next 8 (a,b,c,d,e,f,g) 
8 is abcdefg a -> (c,f), b -> f, c -> (a,f), d -> (a,c,f)

So here's an algorithm...

Start with a map of a to g, each contains the set a to g 
The goal is to reduce this to one element in each set which is the final mapping

Mapping of origin segment to new segment

a abcdefg
b abcdefg   
c abcdefg   
d abcdefg   
e abcdefg   
f abcdefg   
g abcdefg  

Mapping of digit to segment. May be helpful to sort by unique and increasing size 

0 abcefg
1 cf
2 acdeg
3 acdfg
4 bcdf
5 abdfg
6 abdefg
7 acf
8 abcdefg
9 abcdfg

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

a cf
b cf   
c abcdefg   
d acf  
e bcdf   
f bcdf   
g abcdefg  

this gives us a nice starting point but may be nice to have a general algorithm that just works

then combine with this initial step

start with a map of candidates 


















