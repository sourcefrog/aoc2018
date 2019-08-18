# AoC 2018 #23



Part two requires finding the position that's in range of the largest number
of nanobots.

There are a thousand nanobots in the input, distributed over a fairly wide
range of inputs (coordinates on the order of ~1M points) and also with large
ranges. So it seems any kind of brute force search on individual points is
infeasible.

One approach would be to track the set of constraints on the intersection
between the zone of two bots, and then gradually see if we can also reach
any other bots.

So the question then is, is there a tractable representation of the shape of
the intersection between two bots? And more than two bots?

In 2D the Manhattan-distance space will be a diamond-shape with edges on
slope x+y = +/- 1. Similarly in 3d, with planes of unit slope.

Since we're always looking for the intersection of these constraints, I
suspect the constraints will always keep it a simple convex diamond.

Then after having a way to define these shapes, we have to look for the
largest subset of bots having a non-empty intersection. The naive approach
would be to test all `2**1000` possibilities, which is also infeasible. But,
actually, we can often terminate early if we find there is no intersection
for some subset. And we can abandon some possibilites where they cannot
possibly become the longest.

A Bot {x,y,z,r} can reach points (X,Y,Z) where

     (X-x).abs() + (Y-y).abs() + (Z-z).abs() <= r

Start with Z=z to reduce it to the 2D case. Also, start at the X>x, Y>y
case, so the abs terms go away.

     (X - x) + (Y - y) <= r X + Y <= r + x + y

then if X<x, Y>y (x-X) + (Y-y) <= r -X + Y <= r - x + y

similarly X - Y <= r + x - y -X - Y <= r - x - y

These four constraints define a quadrilateral with unit slopes in a plane of
Z=z.

Expanding to Z>z,

       X + Y + Z   <= r + x + y + z
      -X + Y + Z   <= r - x + y + z
       X - Y + Z   <= r x - y + z
      ...

So there are eight planes constraining the space, and they're all defined by
simple combinations of (x,y,z,r). Of course, they have to be.

How do could we intersect two zones to find whether there is any resulting
zone?  First consider the simple one-dimensional case, (x1,r1) and (x2,r2)
where x1 <= x2.

If (x1+r1) < (x2-r2) they do not touch; there is no intersection.

     aaaaAaaaa             x1 = 4, r1 = 4
           bbbbbbBbbbbbb   x2 = 12, r2 = 6

Otherwise, there is an intersection of length di=((x1+r1) - (x2-r2)).  and
with radius (treating the edge as included) of ri = di/2.  Implies r1 = (x1
+ r1 - x2 + r2) / 2.  The center is at (x1 + r1 - ri).

This needs care with regard to off-by-one errors. How do we cope if di is
even? Maybe in that case it cannot be represented as (x,y,z,r)?

And, in fact, this has another unhandled edge case: suppose x1=x2 but r2>r1.

Perhaps it is actually easiest to represent zones as the inclusive ranges of
coordinates, and then at least the math to calculate the intersections is
simple.

So in the case given above, for the A range, x>=0, x<=8. For B, x>=6, x<=18.
The intersection is simply x>=6, x<=8. In other words ximin=max(xamin,
xbmin). ximax=min(xamax, xbmax).

Instead of using both >= and <= we could say: A(x <= 8, -x <= 0). B(x <= 18,
-x <= -6).

How does this extend to 2, and to 3, dimensions?

Then for 2 dimensions give maximum values of (x+y), (-x+y), (x-y), (-x-y).

Similarly for 3 dimesions, the maximum values of all eight combinations.

Perhaps there's a simpler expression than writing them all out. Not sure.

Now, how can we tell if the zone is empty? Equivalently, r<0?

     pxpypz = x + y + z + r mxmymz = -x - y - z + r

     pxpypz + mxmymz = 2r

Now, moving on to finding the zone, and the closest-to-origin point, that
intersects the zones of the largest number of bots.

One way to approach this is to find the zones that intersect between all
combinations of bots, pruning off combinations that have no intersection.
The problem with this is that there are up to `2**1000` possible combinations
of bots, and there are many overlaps between them, so not much can be pruned
out.

We could take the same approach but work from the other end and see how many
need to be removed to find an intersection, although that might also blow
up.

I wonder if it would help to cluster bots at similar locations, or deal
first with the largest ones?

So we can consider this as a function over x,y,z where h(x,y,z) gives the
number of bots in range at any point. We want to find the point with the
maximum x,y,z. The values can be considered to stack up as we go along.

There are vary many possible x,y,z because the ranges are so large, but only
some of them can be interesting: the ones at the edge of a zone boundary. No
new ones can possibly be introduced.

(In fact, is it only the edges closest to the origin that are interesting?)

Each bot defines a diamond with planes defined by eight parameters. The
diamond has six corners.  Are the interesting points always at one of these
corners? If so, perhaps it's easy to see which other bots might contain
them.

82010378 is too low...

Just checking the corners, it seems, is not enough: it gets an answer that
is too low (too close to the origin.) Perhaps, there's another position
further away that intersects with more diamonds.

Perhaps checking only the corners is not enough: when there are multiple
overlapping squares I can roughly imagine how the most-covered point
wouldn't be exactly on a corner. Must it be on an edge? It would be nice to
have a rigorous argument that it must.

Return to the concept of drawing a graph along one axis, and looking for the
maximum stacking. It still seems it must occur on corners?

I wonder if I should be looking for intersections between all the planes,
rather than just the corners? But, I can't yet think of any case where that
demonstrates any difference, or where the positions could be other than at
all the corners.

...

Since I can't work out what's wrong with this, here's one way to make
progress: use a seeded PRNG to generate a scenario with many bots, but
not so many or over such a large space that we can't test it exhaustively.

If all the bots are within say the `-50..50` range on each axis, then we could
check every cell and that would be only `100**3` or `1_000_000` positions to
check, which is feasible.

OK, so that test, with a bit of fiddling, did find a point where they disagree!
Great! Go me! And, as intended, it's reproducible:

    [src/bin/aoc23.rs:394] quick = (
        41,
        (
            -4,
            -34,
            -3,
        ),
    )
    [src/bin/aoc23.rs:396] slow = (
        54,
        (
            -35,
            16,
            -3,
        ),
    )
    thread 'tests::fuzz' panicked at 'assertion failed: `(left == right)`
    left: `41`,
    right: `54`', src/bin/aoc23.rs:397:9
