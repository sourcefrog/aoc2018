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

## Part 2

This also seems like a shortest-path question, but with the additional
complication of choosing the right tool.

We could, at least conceptually, model that as an additional z-dimension
on the map, with three levels depending on which tool is equipped. You
can move to another square with the same tool equipped (if that is allowed)
or you can stay in the same (x,y) location and move to a different
tool, also at a cost.

So perhaps this is then just Djikstra's algorithm: remember the shortest
known cost to reach each `(x, y, t)` position.

I thought I already had a generic implementation of this, but perhaps not.

We could compute ground values on demand, memoized. Or perhaps it's simple
enough to just pre-compute a matrix 3x3 what we need?
