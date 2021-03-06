# Advent of Code - Day 8

A horribly convoluted implementation in Rust of this problem...

https://adventofcode.com/2021/day/8

The solution and my working is below, but there is a much simpler solution as shown in this Reddit thread.

https://www.reddit.com/r/adventofcode/comments/rcnqnr/2021_day_8_part_2_quick_logic_no_math/

TODO My code makes a lot of obnoxious use of clone. Some of that can be mitigated by using persistent data structures such as those implemented in this crate.

https://github.com/orium/rpds


```
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
```

Digits and the number of segments each has:

```
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
```

Insights...

A set of wires in the input must be mapped to a digit 
Some can be deduced because there is a unique number of digits

```
6,5,5,5,6,6
2,3,4,7
```

Then three 6's and 5's.

Digits with uniques are 1(2),7(3),4(4) and 8(7).

Part one is to simply parse the input and count the number of elements with uniques.

Next step, we want to fill out a map of new digit to original digit. 

For example:

```
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

a -> c
b -> f
c -> g
d -> a
e -> b
f -> d
g -> e
```

`acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf`

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

start with a map of candidates grouped by length
here I show the digit it could be and the original segments

len 2 (1, cf)
ab

len 3 (7, acf)
dab

len 4 (4, bcdf)
eafb


len 5 (2,3 or 5, acdeg, acdfg or abdfg)
cdfbe
gcdfa 
fbcad

len 6 (6,9 or 0, abdefg, abcdfg, abcefg)
cefabd
cdfgeb
cagedb

len 7 (can only be 8)
acedgfb

So here's the algorithm, given the list of segments by length and a
candidate mapping recursively assign, and each call returns None for failure
or success with a solution mapping which can then be interpreted to get the 
digits. 

```
fn assign_mapping(input: Vec<HashSet<char>>, // list of inputs sorted by len 
    candidate_map: HashMap<char,HashSet<char>>, // current map of new digit to originals
    digit_segments: HashMap<u8, HashSet<char>>, // map of digits to the original segments they use
NOT NEEDED    digit_lengths: HashMap<u8, Vec<u8>>, // map of lengths of digits so we can look up candidates,
    digits_unused: HashSet<u8> // digits we have not yet assigned
    ) 
    -> Option<HashSet<char>>)
```

assign mapping ()
check for victory (in which candidate map has only mapping in each digit)

take the input head
  ab 
  length is 2 
  find all digits in digit unused where digit_segments has same length (2)
  panic if not found though
   for each candidate digit (1)
      
	  let segments = lookup from digit_segemnts (for 1 returns cf)
      let result = assign_mapping(tail of input,
         update candidate map(ab, cf, candidate_map),
         digit_segments,
         digit_lengths,
         digits used with candidate (1) removed)
      if result is not none return result and party
	  
   end
   
   return none
end

```
fn update_candidate_map(
    new_digits: Vec<char>,
    candidates: Vec<char>,
    candidate_map: HashMap<char,HashSet<char>>, // current map of new digit to originals
    ) 
    -> HashMap<char,HashSet<char>>
```
e.g we get cf, ab, candidate_map

what we do is take the intersection of the candidate map for each new digit 

so whatever c was it will be ab and f will be ab too.

we return the new map 

this then has to be used in a brute force reduction of the candidates 

example 

a { b,c} b { b,c } c { a,b,c }

set the output map to empty 

assign(input map, outputmap) outputmap

  if output map is a winner return it 
  else

  take a key (a)
    for each candidate (b,c) 
       assign(map without key a, candidate map has a -> b)
	   if returns Some we're done
	   otherwise continue


a { b,c} b { b,c } c { a,b,c }

when you assign a to b in the first step
a -> b
we also must discount b for the remaining choices

two ways to do that , a hashset of assigned segments
search the map for the candidate 

second is easier for now

target

a -> c
b -> f
c -> g
d -> a
e -> b
f -> d
g -> e

actual

{'e': 'd'
 'a': 'c'
 'b': 'f'
 'c': 'g'
 'f': 'b'
 'g': 'e'
 'd': 'a'}
 
 cdfeb should be 5
 gabdf 
 
 sometimes works sometimes fails
 
fail
{'a': 'f', 'c': 'g', 'g': 'e', 'd': 'a', 'b': 'c', 'e': 'b', 'f': 'd'}
remapped digits [{'c', 'g', 'd', 'a', 'b'}, {'d', 'c', 'f', 'g', 'a'}, {'b', 'g', 'c', 'd', 'a'}, {'c', 'g', 'f', 'a', 'd'}]
thread 'part2_test_pattern' panicked at 'No digit with segments {'c', 'g', 'd', 'a', 'b'}', src/main.rs:231:18

works
{'f': 'd', 'g': 'e', 'a': 'c', 'b': 'f', 'e': 'b', 'd': 'a', 'c': 'g'}
remapped digits [{'d', 'g', 'b', 'a', 'f'}, {'d', 'c', 'g', 'a', 'f'}, {'a', 'd', 'f', 'b', 'g'}, {'d', 'g', 'f', 'c', 'a'}]
digits [5, 3, 5, 3]

need to find all valid solutions, run it through the digit conversion and see if it works

Final note, yep that worked.
