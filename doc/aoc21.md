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

Means just r3 = 65536 (i.e. 1 << 16).

    7  seti 10828530 0 5   r5 = 10828530
    bani 3 255 2
    addr 5 2 5
    bani 5 16777215 5
    muli 5 65899 5
    bani 5 16777215 5
    gtir 256 3 2
    14  addr 2 4 4      r4 += r2; conditional skip
    15  addi 4 1 4      Skip following?
    16  seti 27 4 4     Jump to after 27
    17  seti 0 4 2
    18  addi 2 1 1
    19  muli 1 256 1
    20  gtrr 1 3 1
    21  addr 1 4 4      r4 += r1; conditional skip one
    22  addi 4 1 4      skip following
    23  seti 25 9 4     Jump to after 25
    24  addi 2 1 2      r2 += 1
    25  seti 17 9 4     Jump after 17
    26  setr 2 8 3      r3 = r2
    27  seti 7 9 4      jump after 7
    28  eqrr 5 0 2      r2 == (r5 == r0)
    29  addr 2 4 4      r4 += r2; conditional skip
    30  seti 5 5 4      Jump after 5

Executing line 29 will jump past the last instruction, and thereby halt. Can it halt
anywhere else? 

This can't be reached in a straight line because of the unconditional jump at 27. 
But 16 jumps directly there. So we'll stop if at this point r5==r0.

r0 seems to never be assigned, it just keeps the initial value. That's nice.