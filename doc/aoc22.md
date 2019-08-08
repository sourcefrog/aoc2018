# aoc 22

Keep a matrix of region cells, each of which has a type Rocky, Narrow, or Wet.

Inputs are the *depth* and the *coordinates* of the target.

Need to first determine the *geologic index* which it seems
we can recursively calculate from the knowing the *target*.

Then we can compute the *erosion level* of each cell
as just the *geologic index*
plus the *cave system depth* modulo 20183, and from that the *region type*.

Coordinates are represented as (x, y) going down and to the right.

The total risk is the sum of the risk levels determined by the type of
each region, from (0,0) down to the target, inclusive.

So, we only need to calculate values for that rectangle. That seems
pretty simple?

...

OK so one complication is that the geologic index numbers get quite
large, because so many numbers are multiplied together. Perhaps we
shouldn't save the whole geologic index but rather compute
the derived values as we go. Is this possible? Or, alternatively and
somewhat lazily, use a rust bignum type.

All we ultimately really need is the region type, derived from the erosion
level, derived from the geologic index (plus the depth.)

Oh, actually I misread this: it compounds not by multiplying the geologic
indexes, but the erosion levels. And the erosion levels are capped.
