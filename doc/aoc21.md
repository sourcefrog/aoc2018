# AoC 21

    #ip 4
    0  seti 123 0 5
    1  bani 5 456 5
    2  eqri 5 72 5

Check that 123 & 456 = 72 (as it is in bitwise arithmetic, with those decimal inputs.)

    3  addr 5 4 4          r4 = r4 + r5

If previous check was true, skip following instruction

    4  seti 0 0 4          r4 = 0

If bani test was false, go back to 0, in an infinite loop.

    5  seti 0 9 5          r5 = 0

Continue here if test was OK; reset r5.

     6  bori 5 65536 3      r3 = r5 | 65536
        ## Since r5 = 0 sets r3 = 65536 (i.e. 1 << 16).
     7  seti 10828530 0 5   r5 = 10828530
     8  bani 3 255 2        r2 = r3 & 255   # reached from 27
     9  addr 5 2 5
    10  bani 5 16777215 5
    11  muli 5 65899 5
    12  bani 5 16777215 5
    13  gtir 256 3 2
    14  addr 2 4 4           r4 += r2; conditional skip
    15  addi 4 1 4           Skip following?
    16  seti 27 4 4          Jump to after 27
    17  seti 0 4 2           r2 = 0
    18  addi 2 1 1           r1 = r2 + 1
    19  muli 1 256 1         r1 = r1 * 256
    20  gtrr 1 3 1           r1 = (r1 > r3)
    21  addr 1 4 4           r4 += r1; conditional skip one
    22  addi 4 1 4           skip following
    23  seti 25 9 4          Jump to after 25
    24  addi 2 1 2           r2 += 1
    25  seti 17 9 4          Jump after 17
    26  setr 2 8 3           r3 = r2
    27  seti 7 9 4           jump after 7
    28  eqrr 5 0 2           r2 == (r5 == r0)
    29  addr 2 4 4           r4 += r2; conditional skip
    30  seti 5 5 4           Jump after 5

Executing line 29 will jump past the last instruction, and thereby halt. Can it
halt anywhere else?

This can't be reached in a straight line because of the unconditional jump at
27. But it can be reached from 16. So we'll stop if at this point r5==r0.

r0 seems to never be assigned, it just keeps the initial value. That's nice.

So what will r5 be there? It looks like r5 is only updated around the
region of 7-12.

Rather than solving this in closed form I could run this code using the
existing interpreter, and see what values of r5 we ever observe at line 28.
That gives the correct answer of 202209.

## Part Two

> What is the lowest non-negative integer value for register 0 that causes the
> program to halt after executing the most instructions? (The program must
> actually halt; running forever does not count as halting.)

The previous empirical approach seems that we find many many unique r5 values
without reaching any obvious plateau and also without repeating, so this might
need a bit more thought.

Actually: the numbers are always under 16777215, 0xffffff, because of the
two `bani` instructions that cap it, and that's not unreasonable to just test
by brute force, even if it takes a few seconds to run.

I'm not totally convinced why this must be true, but the first time we find a
repeated value, that means the system is cycling. Whatever was the last value
we saw before it started to cycle, is the longest answer.

The answer was 11777564.