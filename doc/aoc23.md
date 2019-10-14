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
for some subset. And we can abandon some possibilities where they cannot
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

Similarly for 3 dimensions, the maximum values of all eight combinations.

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

...

Let's return to the idea of gradually building up points that intersect
multiple zones, but keep it simpler by just remembering all the known zones
that intersect at least `N` bots.

...

In `6a30de5` I tried generating the set of zones covered by `i` out of the
first `j` bots. This seems to be too slow to do at large scale: we generate
thousands of lightly-covered zones and spend too long updating them.

One incremental approach on this is: only update the thousand most interesting
zones, on each round. The ones that are only lightly covered are probably
unlikely to come back.

Other ideas:

* Look first at which bots are within range of each other *at all*, without
  worrying about just where they intersect.  Then look in more detail for the
  intersections between them. It might still be too much.

* Think again about some kind of closed-form representation of the zones as
  functions over the `pxpypz` space.

So limiting it to considering 1000 new intersections per bot, does get it to
converge in a reasonable amount of time, eventually finding a zone that's
covered by about 975 bots.

Now we'd just need to look for the point closest to the origin within that
space.

If we could draw a number line on each of those variables with steps up and
down on it, then we could accumulate the steps in `O(bots)` and walk along it
also in `O(bots)` to find the highest point. The only difficulty is that
they're tangled together, but perhaps that can be rearranged with a little
algebra.

The zone is

    x + y + z <= pxpypz
    x - y - z <= pxmymz
    ...

Therefore

    2x  <= pxpypz + pxmymz
    -2x <= mxpypz + mxmymz

Or rather, just around a bot diamond centered at `(X, Y, Z)`:

    x >= X - r
    x <= X + r

...

We could look for a plane in `x + y + z` that crosses the largest number of
bots, but the problem with this is that there's no guarantee that all those
bots actually intersect with each other.

...

Let's pause and see how many bots overlap at all.

It turns out they all overlap with at least one other bot, but not all overlap
with all.  And, interestingly, many of them overlap very richly, and a few are
not very connected at all. Perhaps this can be used to reduce the number we
actually need to look at and make it easier to control.

    bot    0 touches  987 bots
    bot    1 touches  985 bots
    bot    2 touches  982 bots
    bot    3 touches  993 bots
    bot    4 touches  983 bots
    bot    5 touches  982 bots
    bot    6 touches  983 bots
    bot    7 touches  984 bots
    bot    8 touches  985 bots
    bot    9 touches  993 bots
    bot   10 touches  986 bots
    bot   11 touches  983 bots
    bot   12 touches  990 bots
    bot   13 touches  987 bots
    bot   14 touches  982 bots
    bot   15 touches  992 bots
    bot   16 touches  984 bots
    bot   17 touches  982 bots
    bot   18 touches  983 bots
    bot   19 touches  989 bots
    bot   20 touches  982 bots
    bot   21 touches  990 bots
    bot   22 touches  983 bots
    bot   23 touches  982 bots
    bot   24 touches  986 bots
    bot   25 touches  992 bots
    bot   26 touches  989 bots
    bot   27 touches  986 bots
    bot   28 touches  991 bots
    bot   29 touches  987 bots
    bot   30 touches  982 bots
    bot   31 touches  983 bots
    bot   32 touches  985 bots
    bot   33 touches  982 bots
    bot   34 touches  986 bots
    bot   35 touches  986 bots
    bot   36 touches  982 bots
    bot   37 touches  984 bots
    bot   38 touches  982 bots
    bot   39 touches  986 bots
    bot   40 touches  982 bots
    bot   41 touches  982 bots
    bot   42 touches  984 bots
    bot   43 touches  989 bots
    bot   44 touches  985 bots
    bot   45 touches  121 bots
    bot   46 touches  986 bots
    bot   47 touches  982 bots
    bot   48 touches  987 bots
    bot   49 touches  992 bots
    bot   50 touches  986 bots
    bot   51 touches  986 bots
    bot   52 touches  983 bots
    bot   53 touches  986 bots
    bot   54 touches  983 bots
    bot   55 touches  985 bots
    bot   56 touches  982 bots
    bot   57 touches  982 bots
    bot   58 touches  984 bots
    bot   59 touches  986 bots
    bot   60 touches  985 bots
    bot   61 touches  982 bots
    bot   62 touches  982 bots
    bot   63 touches  982 bots
    bot   64 touches  982 bots
    bot   65 touches  982 bots
    bot   66 touches  984 bots
    bot   67 touches  986 bots
    bot   68 touches  985 bots
    bot   69 touches  989 bots
    bot   70 touches  983 bots
    bot   71 touches  982 bots
    bot   72 touches  987 bots
    bot   73 touches  983 bots
    bot   74 touches  984 bots
    bot   75 touches  984 bots
    bot   76 touches  986 bots
    bot   77 touches  982 bots
    bot   78 touches  983 bots
    bot   79 touches  982 bots
    bot   80 touches  984 bots
    bot   81 touches  985 bots
    bot   82 touches  982 bots
    bot   83 touches  984 bots
    bot   84 touches  986 bots
    bot   85 touches  982 bots
    bot   86 touches  982 bots
    bot   87 touches  162 bots
    bot   88 touches  982 bots
    bot   89 touches  993 bots
    bot   90 touches  984 bots
    bot   91 touches  987 bots
    bot   92 touches  983 bots
    bot   93 touches  985 bots
    bot   94 touches  993 bots
    bot   95 touches  982 bots
    bot   96 touches  984 bots
    bot   97 touches  982 bots
    bot   98 touches  982 bots
    bot   99 touches  271 bots
    bot  100 touches  984 bots
    bot  101 touches  982 bots
    bot  102 touches  984 bots
    bot  103 touches  986 bots
    bot  104 touches  982 bots
    bot  105 touches  985 bots
    bot  106 touches  982 bots
    bot  107 touches  985 bots
    bot  108 touches   36 bots
    bot  109 touches  982 bots
    bot  110 touches  982 bots
    bot  111 touches  982 bots
    bot  112 touches  983 bots
    bot  113 touches  982 bots
    bot  114 touches  982 bots
    bot  115 touches  982 bots
    bot  116 touches  985 bots
    bot  117 touches  989 bots
    bot  118 touches  984 bots
    bot  119 touches  984 bots
    bot  120 touches  989 bots
    bot  121 touches  983 bots
    bot  122 touches  982 bots
    bot  123 touches  982 bots
    bot  124 touches  982 bots
    bot  125 touches  982 bots
    bot  126 touches  982 bots
    bot  127 touches  985 bots
    bot  128 touches  982 bots
    bot  129 touches  982 bots
    bot  130 touches  982 bots
    bot  131 touches  984 bots
    bot  132 touches  982 bots
    bot  133 touches  982 bots
    bot  134 touches  982 bots
    bot  135 touches  982 bots
    bot  136 touches  988 bots
    bot  137 touches  984 bots
    bot  138 touches  985 bots
    bot  139 touches  983 bots
    bot  140 touches  985 bots
    bot  141 touches  983 bots
    bot  142 touches  985 bots
    bot  143 touches  982 bots
    bot  144 touches  983 bots
    bot  145 touches  993 bots
    bot  146 touches  991 bots
    bot  147 touches  984 bots
    bot  148 touches  985 bots
    bot  149 touches  988 bots
    bot  150 touches  983 bots
    bot  151 touches  989 bots
    bot  152 touches  986 bots
    bot  153 touches  982 bots
    bot  154 touches  985 bots
    bot  155 touches  988 bots
    bot  156 touches  984 bots
    bot  157 touches  982 bots
    bot  158 touches  982 bots
    bot  159 touches  982 bots
    bot  160 touches  982 bots
    bot  161 touches  983 bots
    bot  162 touches  983 bots
    bot  163 touches  984 bots
    bot  164 touches  984 bots
    bot  165 touches  983 bots
    bot  166 touches  984 bots
    bot  167 touches  984 bots
    bot  168 touches  982 bots
    bot  169 touches  986 bots
    bot  170 touches  987 bots
    bot  171 touches  992 bots
    bot  172 touches  983 bots
    bot  173 touches  982 bots
    bot  174 touches  982 bots
    bot  175 touches  987 bots
    bot  176 touches  990 bots
    bot  177 touches  982 bots
    bot  178 touches  990 bots
    bot  179 touches  984 bots
    bot  180 touches  985 bots
    bot  181 touches  988 bots
    bot  182 touches  982 bots
    bot  183 touches  983 bots
    bot  184 touches  987 bots
    bot  185 touches  988 bots
    bot  186 touches  983 bots
    bot  187 touches  988 bots
    bot  188 touches  986 bots
    bot  189 touches  988 bots
    bot  190 touches  982 bots
    bot  191 touches  984 bots
    bot  192 touches  984 bots
    bot  193 touches  991 bots
    bot  194 touches  982 bots
    bot  195 touches  982 bots
    bot  196 touches  982 bots
    bot  197 touches  991 bots
    bot  198 touches  990 bots
    bot  199 touches  982 bots
    bot  200 touches  985 bots
    bot  201 touches  984 bots
    bot  202 touches  523 bots
    bot  203 touches  984 bots
    bot  204 touches  986 bots
    bot  205 touches  991 bots
    bot  206 touches  987 bots
    bot  207 touches  982 bots
    bot  208 touches  984 bots
    bot  209 touches  989 bots
    bot  210 touches  991 bots
    bot  211 touches  207 bots
    bot  212 touches  982 bots
    bot  213 touches  982 bots
    bot  214 touches  983 bots
    bot  215 touches  982 bots
    bot  216 touches  991 bots
    bot  217 touches  983 bots
    bot  218 touches  988 bots
    bot  219 touches  982 bots
    bot  220 touches  987 bots
    bot  221 touches  988 bots
    bot  222 touches  983 bots
    bot  223 touches  983 bots
    bot  224 touches  986 bots
    bot  225 touches  982 bots
    bot  226 touches  987 bots
    bot  227 touches  992 bots
    bot  228 touches  989 bots
    bot  229 touches  982 bots
    bot  230 touches  982 bots
    bot  231 touches  993 bots
    bot  232 touches  985 bots
    bot  233 touches  987 bots
    bot  234 touches  982 bots
    bot  235 touches  982 bots
    bot  236 touches  985 bots
    bot  237 touches  982 bots
    bot  238 touches  987 bots
    bot  239 touches  982 bots
    bot  240 touches  984 bots
    bot  241 touches  984 bots
    bot  242 touches  987 bots
    bot  243 touches  982 bots
    bot  244 touches  982 bots
    bot  245 touches  984 bots
    bot  246 touches  982 bots
    bot  247 touches   50 bots
    bot  248 touches  983 bots
    bot  249 touches  982 bots
    bot  250 touches  983 bots
    bot  251 touches  983 bots
    bot  252 touches  984 bots
    bot  253 touches  982 bots
    bot  254 touches  983 bots
    bot  255 touches  982 bots
    bot  256 touches  988 bots
    bot  257 touches  982 bots
    bot  258 touches  982 bots
    bot  259 touches  982 bots
    bot  260 touches  984 bots
    bot  261 touches  982 bots
    bot  262 touches  982 bots
    bot  263 touches  983 bots
    bot  264 touches  982 bots
    bot  265 touches  982 bots
    bot  266 touches  992 bots
    bot  267 touches  992 bots
    bot  268 touches  982 bots
    bot  269 touches  982 bots
    bot  270 touches  983 bots
    bot  271 touches  985 bots
    bot  272 touches  983 bots
    bot  273 touches  982 bots
    bot  274 touches  986 bots
    bot  275 touches  991 bots
    bot  276 touches  983 bots
    bot  277 touches  982 bots
    bot  278 touches  982 bots
    bot  279 touches  982 bots
    bot  280 touches  984 bots
    bot  281 touches  985 bots
    bot  282 touches  983 bots
    bot  283 touches  982 bots
    bot  284 touches  992 bots
    bot  285 touches  984 bots
    bot  286 touches  983 bots
    bot  287 touches  982 bots
    bot  288 touches  983 bots
    bot  289 touches  992 bots
    bot  290 touches  984 bots
    bot  291 touches  982 bots
    bot  292 touches  982 bots
    bot  293 touches  166 bots
    bot  294 touches  992 bots
    bot  295 touches  988 bots
    bot  296 touches  985 bots
    bot  297 touches  985 bots
    bot  298 touches  986 bots
    bot  299 touches  982 bots
    bot  300 touches  985 bots
    bot  301 touches  983 bots
    bot  302 touches  988 bots
    bot  303 touches  984 bots
    bot  304 touches  984 bots
    bot  305 touches  982 bots
    bot  306 touches  988 bots
    bot  307 touches  982 bots
    bot  308 touches  982 bots
    bot  309 touches  985 bots
    bot  310 touches  989 bots
    bot  311 touches  985 bots
    bot  312 touches  984 bots
    bot  313 touches  982 bots
    bot  314 touches  984 bots
    bot  315 touches  984 bots
    bot  316 touches  983 bots
    bot  317 touches  984 bots
    bot  318 touches  986 bots
    bot  319 touches  993 bots
    bot  320 touches  983 bots
    bot  321 touches  986 bots
    bot  322 touches  989 bots
    bot  323 touches  990 bots
    bot  324 touches  993 bots
    bot  325 touches  987 bots
    bot  326 touches  982 bots
    bot  327 touches  988 bots
    bot  328 touches  989 bots
    bot  329 touches  982 bots
    bot  330 touches  985 bots
    bot  331 touches  984 bots
    bot  332 touches  982 bots
    bot  333 touches  982 bots
    bot  334 touches  989 bots
    bot  335 touches  986 bots
    bot  336 touches  990 bots
    bot  337 touches  983 bots
    bot  338 touches  992 bots
    bot  339 touches  993 bots
    bot  340 touches  985 bots
    bot  341 touches  982 bots
    bot  342 touches  987 bots
    bot  343 touches  982 bots
    bot  344 touches  982 bots
    bot  345 touches  986 bots
    bot  346 touches  983 bots
    bot  347 touches  983 bots
    bot  348 touches  989 bots
    bot  349 touches  986 bots
    bot  350 touches  982 bots
    bot  351 touches  985 bots
    bot  352 touches  988 bots
    bot  353 touches  988 bots
    bot  354 touches  982 bots
    bot  355 touches  985 bots
    bot  356 touches  992 bots
    bot  357 touches  984 bots
    bot  358 touches  982 bots
    bot  359 touches  984 bots
    bot  360 touches  988 bots
    bot  361 touches  984 bots
    bot  362 touches  982 bots
    bot  363 touches  984 bots
    bot  364 touches  986 bots
    bot  365 touches  987 bots
    bot  366 touches  984 bots
    bot  367 touches  983 bots
    bot  368 touches  987 bots
    bot  369 touches  982 bots
    bot  370 touches  984 bots
    bot  371 touches  991 bots
    bot  372 touches  988 bots
    bot  373 touches  986 bots
    bot  374 touches  986 bots
    bot  375 touches  986 bots
    bot  376 touches  991 bots
    bot  377 touches  993 bots
    bot  378 touches  984 bots
    bot  379 touches  983 bots
    bot  380 touches  985 bots
    bot  381 touches  983 bots
    bot  382 touches  982 bots
    bot  383 touches  985 bots
    bot  384 touches  984 bots
    bot  385 touches  990 bots
    bot  386 touches  982 bots
    bot  387 touches  982 bots
    bot  388 touches  982 bots
    bot  389 touches  982 bots
    bot  390 touches  986 bots
    bot  391 touches  982 bots
    bot  392 touches  983 bots
    bot  393 touches  983 bots
    bot  394 touches  988 bots
    bot  395 touches  984 bots
    bot  396 touches  982 bots
    bot  397 touches  984 bots
    bot  398 touches  986 bots
    bot  399 touches  984 bots
    bot  400 touches  993 bots
    bot  401 touches  989 bots
    bot  402 touches  983 bots
    bot  403 touches  982 bots
    bot  404 touches  982 bots
    bot  405 touches  982 bots
    bot  406 touches  982 bots
    bot  407 touches  985 bots
    bot  408 touches  982 bots
    bot  409 touches  984 bots
    bot  410 touches  982 bots
    bot  411 touches  982 bots
    bot  412 touches  990 bots
    bot  413 touches  984 bots
    bot  414 touches  982 bots
    bot  415 touches  989 bots
    bot  416 touches  983 bots
    bot  417 touches  986 bots
    bot  418 touches  987 bots
    bot  419 touches  984 bots
    bot  420 touches  991 bots
    bot  421 touches  989 bots
    bot  422 touches  984 bots
    bot  423 touches  984 bots
    bot  424 touches  986 bots
    bot  425 touches  983 bots
    bot  426 touches  982 bots
    bot  427 touches  983 bots
    bot  428 touches  992 bots
    bot  429 touches  987 bots
    bot  430 touches  982 bots
    bot  431 touches  986 bots
    bot  432 touches  985 bots
    bot  433 touches  985 bots
    bot  434 touches  983 bots
    bot  435 touches  982 bots
    bot  436 touches  982 bots
    bot  437 touches  985 bots
    bot  438 touches  982 bots
    bot  439 touches  987 bots
    bot  440 touches  982 bots
    bot  441 touches  986 bots
    bot  442 touches  987 bots
    bot  443 touches  982 bots
    bot  444 touches  983 bots
    bot  445 touches  982 bots
    bot  446 touches  987 bots
    bot  447 touches  983 bots
    bot  448 touches  986 bots
    bot  449 touches  986 bots
    bot  450 touches  983 bots
    bot  451 touches  990 bots
    bot  452 touches  991 bots
    bot  453 touches  982 bots
    bot  454 touches  984 bots
    bot  455 touches  984 bots
    bot  456 touches  985 bots
    bot  457 touches  985 bots
    bot  458 touches  989 bots
    bot  459 touches  991 bots
    bot  460 touches  984 bots
    bot  461 touches  982 bots
    bot  462 touches  983 bots
    bot  463 touches  983 bots
    bot  464 touches  982 bots
    bot  465 touches  982 bots
    bot  466 touches  983 bots
    bot  467 touches  986 bots
    bot  468 touches  983 bots
    bot  469 touches  982 bots
    bot  470 touches  983 bots
    bot  471 touches  983 bots
    bot  472 touches  982 bots
    bot  473 touches  987 bots
    bot  474 touches  984 bots
    bot  475 touches  982 bots
    bot  476 touches  982 bots
    bot  477 touches  983 bots
    bot  478 touches  984 bots
    bot  479 touches  982 bots
    bot  480 touches  982 bots
    bot  481 touches  991 bots
    bot  482 touches  992 bots
    bot  483 touches  983 bots
    bot  484 touches  990 bots
    bot  485 touches  982 bots
    bot  486 touches  984 bots
    bot  487 touches  989 bots
    bot  488 touches  982 bots
    bot  489 touches  985 bots
    bot  490 touches  983 bots
    bot  491 touches  984 bots
    bot  492 touches  984 bots
    bot  493 touches  982 bots
    bot  494 touches  983 bots
    bot  495 touches  982 bots
    bot  496 touches  982 bots
    bot  497 touches  986 bots
    bot  498 touches  984 bots
    bot  499 touches  982 bots
    bot  500 touches  984 bots
    bot  501 touches  989 bots
    bot  502 touches  982 bots
    bot  503 touches  982 bots
    bot  504 touches  987 bots
    bot  505 touches  983 bots
    bot  506 touches  984 bots
    bot  507 touches  983 bots
    bot  508 touches  989 bots
    bot  509 touches  988 bots
    bot  510 touches  982 bots
    bot  511 touches  984 bots
    bot  512 touches  982 bots
    bot  513 touches  982 bots
    bot  514 touches  982 bots
    bot  515 touches  985 bots
    bot  516 touches  991 bots
    bot  517 touches  989 bots
    bot  518 touches  982 bots
    bot  519 touches  989 bots
    bot  520 touches  984 bots
    bot  521 touches  191 bots
    bot  522 touches  987 bots
    bot  523 touches  984 bots
    bot  524 touches  984 bots
    bot  525 touches  982 bots
    bot  526 touches  983 bots
    bot  527 touches  983 bots
    bot  528 touches  983 bots
    bot  529 touches  982 bots
    bot  530 touches  982 bots
    bot  531 touches  984 bots
    bot  532 touches  982 bots
    bot  533 touches  983 bots
    bot  534 touches  982 bots
    bot  535 touches  987 bots
    bot  536 touches  985 bots
    bot  537 touches  982 bots
    bot  538 touches  982 bots
    bot  539 touches  984 bots
    bot  540 touches  989 bots
    bot  541 touches  983 bots
    bot  542 touches  139 bots
    bot  543 touches  982 bots
    bot  544 touches  989 bots
    bot  545 touches  983 bots
    bot  546 touches  982 bots
    bot  547 touches  983 bots
    bot  548 touches  982 bots
    bot  549 touches  993 bots
    bot  550 touches  984 bots
    bot  551 touches  983 bots
    bot  552 touches  982 bots
    bot  553 touches  991 bots
    bot  554 touches  983 bots
    bot  555 touches  984 bots
    bot  556 touches  990 bots
    bot  557 touches  988 bots
    bot  558 touches  985 bots
    bot  559 touches  984 bots
    bot  560 touches  988 bots
    bot  561 touches  983 bots
    bot  562 touches  984 bots
    bot  563 touches  986 bots
    bot  564 touches  988 bots
    bot  565 touches  983 bots
    bot  566 touches  982 bots
    bot  567 touches  991 bots
    bot  568 touches   78 bots
    bot  569 touches  983 bots
    bot  570 touches  989 bots
    bot  571 touches  984 bots
    bot  572 touches  982 bots
    bot  573 touches  985 bots
    bot  574 touches  985 bots
    bot  575 touches  986 bots
    bot  576 touches  982 bots
    bot  577 touches  988 bots
    bot  578 touches  989 bots
    bot  579 touches  986 bots
    bot  580 touches  984 bots
    bot  581 touches  982 bots
    bot  582 touches  982 bots
    bot  583 touches  985 bots
    bot  584 touches  985 bots
    bot  585 touches  982 bots
    bot  586 touches  983 bots
    bot  587 touches  987 bots
    bot  588 touches  982 bots
    bot  589 touches  984 bots
    bot  590 touches  983 bots
    bot  591 touches  985 bots
    bot  592 touches  982 bots
    bot  593 touches  993 bots
    bot  594 touches  989 bots
    bot  595 touches  983 bots
    bot  596 touches  990 bots
    bot  597 touches  982 bots
    bot  598 touches  982 bots
    bot  599 touches  986 bots
    bot  600 touches  993 bots
    bot  601 touches  991 bots
    bot  602 touches  991 bots
    bot  603 touches  985 bots
    bot  604 touches  982 bots
    bot  605 touches  987 bots
    bot  606 touches  984 bots
    bot  607 touches  982 bots
    bot  608 touches  982 bots
    bot  609 touches  982 bots
    bot  610 touches  984 bots
    bot  611 touches  982 bots
    bot  612 touches  982 bots
    bot  613 touches  985 bots
    bot  614 touches  983 bots
    bot  615 touches  987 bots
    bot  616 touches  992 bots
    bot  617 touches  986 bots
    bot  618 touches  989 bots
    bot  619 touches  985 bots
    bot  620 touches  987 bots
    bot  621 touches  987 bots
    bot  622 touches  982 bots
    bot  623 touches  984 bots
    bot  624 touches  985 bots
    bot  625 touches  992 bots
    bot  626 touches  983 bots
    bot  627 touches  990 bots
    bot  628 touches  984 bots
    bot  629 touches  984 bots
    bot  630 touches  982 bots
    bot  631 touches  989 bots
    bot  632 touches  989 bots
    bot  633 touches  983 bots
    bot  634 touches  993 bots
    bot  635 touches  992 bots
    bot  636 touches  985 bots
    bot  637 touches  987 bots
    bot  638 touches  983 bots
    bot  639 touches  982 bots
    bot  640 touches  982 bots
    bot  641 touches  982 bots
    bot  642 touches  982 bots
    bot  643 touches  983 bots
    bot  644 touches  984 bots
    bot  645 touches  990 bots
    bot  646 touches  992 bots
    bot  647 touches  989 bots
    bot  648 touches  983 bots
    bot  649 touches  982 bots
    bot  650 touches  987 bots
    bot  651 touches  982 bots
    bot  652 touches  992 bots
    bot  653 touches  992 bots
    bot  654 touches  982 bots
    bot  655 touches   74 bots
    bot  656 touches  984 bots
    bot  657 touches  984 bots
    bot  658 touches  982 bots
    bot  659 touches  982 bots
    bot  660 touches  983 bots
    bot  661 touches  984 bots
    bot  662 touches  993 bots
    bot  663 touches  982 bots
    bot  664 touches  987 bots
    bot  665 touches  984 bots
    bot  666 touches  989 bots
    bot  667 touches  982 bots
    bot  668 touches  983 bots
    bot  669 touches  983 bots
    bot  670 touches  984 bots
    bot  671 touches  982 bots
    bot  672 touches  982 bots
    bot  673 touches  984 bots
    bot  674 touches  982 bots
    bot  675 touches  984 bots
    bot  676 touches  985 bots
    bot  677 touches  983 bots
    bot  678 touches  985 bots
    bot  679 touches  982 bots
    bot  680 touches  985 bots
    bot  681 touches  989 bots
    bot  682 touches  983 bots
    bot  683 touches  983 bots
    bot  684 touches  982 bots
    bot  685 touches  990 bots
    bot  686 touches  985 bots
    bot  687 touches  984 bots
    bot  688 touches  983 bots
    bot  689 touches  982 bots
    bot  690 touches  985 bots
    bot  691 touches  982 bots
    bot  692 touches  989 bots
    bot  693 touches  985 bots
    bot  694 touches  985 bots
    bot  695 touches  991 bots
    bot  696 touches  313 bots
    bot  697 touches  984 bots
    bot  698 touches  982 bots
    bot  699 touches  985 bots
    bot  700 touches  985 bots
    bot  701 touches  982 bots
    bot  702 touches  982 bots
    bot  703 touches  985 bots
    bot  704 touches  982 bots
    bot  705 touches  985 bots
    bot  706 touches  991 bots
    bot  707 touches  983 bots
    bot  708 touches  991 bots
    bot  709 touches  982 bots
    bot  710 touches  988 bots
    bot  711 touches  986 bots
    bot  712 touches  982 bots
    bot  713 touches  984 bots
    bot  714 touches  985 bots
    bot  715 touches  988 bots
    bot  716 touches  991 bots
    bot  717 touches  984 bots
    bot  718 touches  982 bots
    bot  719 touches  983 bots
    bot  720 touches  987 bots
    bot  721 touches  985 bots
    bot  722 touches  982 bots
    bot  723 touches  982 bots
    bot  724 touches  982 bots
    bot  725 touches  982 bots
    bot  726 touches  986 bots
    bot  727 touches  982 bots
    bot  728 touches  982 bots
    bot  729 touches  983 bots
    bot  730 touches  991 bots
    bot  731 touches  983 bots
    bot  732 touches  985 bots
    bot  733 touches  990 bots
    bot  734 touches  992 bots
    bot  735 touches  985 bots
    bot  736 touches  988 bots
    bot  737 touches  988 bots
    bot  738 touches  983 bots
    bot  739 touches  982 bots
    bot  740 touches  984 bots
    bot  741 touches  984 bots
    bot  742 touches  988 bots
    bot  743 touches  982 bots
    bot  744 touches  985 bots
    bot  745 touches  987 bots
    bot  746 touches  982 bots
    bot  747 touches  989 bots
    bot  748 touches  983 bots
    bot  749 touches  982 bots
    bot  750 touches  988 bots
    bot  751 touches  991 bots
    bot  752 touches  983 bots
    bot  753 touches  986 bots
    bot  754 touches  990 bots
    bot  755 touches  985 bots
    bot  756 touches  982 bots
    bot  757 touches  982 bots
    bot  758 touches  986 bots
    bot  759 touches  982 bots
    bot  760 touches  989 bots
    bot  761 touches   57 bots
    bot  762 touches  985 bots
    bot  763 touches  985 bots
    bot  764 touches  987 bots
    bot  765 touches  983 bots
    bot  766 touches  983 bots
    bot  767 touches  985 bots
    bot  768 touches  982 bots
    bot  769 touches  990 bots
    bot  770 touches  983 bots
    bot  771 touches  990 bots
    bot  772 touches  982 bots
    bot  773 touches  982 bots
    bot  774 touches  982 bots
    bot  775 touches  982 bots
    bot  776 touches  983 bots
    bot  777 touches  984 bots
    bot  778 touches  987 bots
    bot  779 touches  992 bots
    bot  780 touches  984 bots
    bot  781 touches  982 bots
    bot  782 touches  982 bots
    bot  783 touches  984 bots
    bot  784 touches  983 bots
    bot  785 touches  982 bots
    bot  786 touches  985 bots
    bot  787 touches  991 bots
    bot  788 touches  984 bots
    bot  789 touches  991 bots
    bot  790 touches  982 bots
    bot  791 touches  982 bots
    bot  792 touches  982 bots
    bot  793 touches  983 bots
    bot  794 touches  983 bots
    bot  795 touches  993 bots
    bot  796 touches  982 bots
    bot  797 touches  982 bots
    bot  798 touches  361 bots
    bot  799 touches  986 bots
    bot  800 touches  984 bots
    bot  801 touches  992 bots
    bot  802 touches  985 bots
    bot  803 touches  991 bots
    bot  804 touches  985 bots
    bot  805 touches  983 bots
    bot  806 touches  983 bots
    bot  807 touches  985 bots
    bot  808 touches  982 bots
    bot  809 touches  982 bots
    bot  810 touches  987 bots
    bot  811 touches  984 bots
    bot  812 touches  983 bots
    bot  813 touches  987 bots
    bot  814 touches  982 bots
    bot  815 touches  991 bots
    bot  816 touches  984 bots
    bot  817 touches  986 bots
    bot  818 touches  991 bots
    bot  819 touches  988 bots
    bot  820 touches  982 bots
    bot  821 touches  982 bots
    bot  822 touches  988 bots
    bot  823 touches  984 bots
    bot  824 touches  985 bots
    bot  825 touches  982 bots
    bot  826 touches  982 bots
    bot  827 touches  983 bots
    bot  828 touches  983 bots
    bot  829 touches  982 bots
    bot  830 touches  989 bots
    bot  831 touches  992 bots
    bot  832 touches  982 bots
    bot  833 touches  983 bots
    bot  834 touches  987 bots
    bot  835 touches  985 bots
    bot  836 touches  982 bots
    bot  837 touches  982 bots
    bot  838 touches  982 bots
    bot  839 touches  984 bots
    bot  840 touches  983 bots
    bot  841 touches  982 bots
    bot  842 touches  982 bots
    bot  843 touches  990 bots
    bot  844 touches  984 bots
    bot  845 touches  982 bots
    bot  846 touches  992 bots
    bot  847 touches  983 bots
    bot  848 touches  984 bots
    bot  849 touches  983 bots
    bot  850 touches  983 bots
    bot  851 touches  983 bots
    bot  852 touches  982 bots
    bot  853 touches  983 bots
    bot  854 touches  983 bots
    bot  855 touches  985 bots
    bot  856 touches  985 bots
    bot  857 touches  982 bots
    bot  858 touches  983 bots
    bot  859 touches  991 bots
    bot  860 touches  989 bots
    bot  861 touches  987 bots
    bot  862 touches  987 bots
    bot  863 touches  991 bots
    bot  864 touches  984 bots
    bot  865 touches  982 bots
    bot  866 touches  992 bots
    bot  867 touches  982 bots
    bot  868 touches  982 bots
    bot  869 touches  982 bots
    bot  870 touches  983 bots
    bot  871 touches  982 bots
    bot  872 touches  984 bots
    bot  873 touches  982 bots
    bot  874 touches  982 bots
    bot  875 touches  985 bots
    bot  876 touches  985 bots
    bot  877 touches  982 bots
    bot  878 touches  992 bots
    bot  879 touches  993 bots
    bot  880 touches  988 bots
    bot  881 touches  982 bots
    bot  882 touches  982 bots
    bot  883 touches  983 bots
    bot  884 touches  987 bots
    bot  885 touches  988 bots
    bot  886 touches  985 bots
    bot  887 touches  983 bots
    bot  888 touches  982 bots
    bot  889 touches  987 bots
    bot  890 touches  982 bots
    bot  891 touches  985 bots
    bot  892 touches  982 bots
    bot  893 touches  987 bots
    bot  894 touches  985 bots
    bot  895 touches  982 bots
    bot  896 touches  985 bots
    bot  897 touches  984 bots
    bot  898 touches  982 bots
    bot  899 touches  991 bots
    bot  900 touches  984 bots
    bot  901 touches  986 bots
    bot  902 touches  984 bots
    bot  903 touches  983 bots
    bot  904 touches  984 bots
    bot  905 touches  982 bots
    bot  906 touches  982 bots
    bot  907 touches  984 bots
    bot  908 touches  982 bots
    bot  909 touches  984 bots
    bot  910 touches  985 bots
    bot  911 touches  985 bots
    bot  912 touches  985 bots
    bot  913 touches  985 bots
    bot  914 touches  983 bots
    bot  915 touches  990 bots
    bot  916 touches  983 bots
    bot  917 touches  983 bots
    bot  918 touches  985 bots
    bot  919 touches  987 bots
    bot  920 touches  984 bots
    bot  921 touches  982 bots
    bot  922 touches  985 bots
    bot  923 touches  984 bots
    bot  924 touches  982 bots
    bot  925 touches  984 bots
    bot  926 touches  987 bots
    bot  927 touches  985 bots
    bot  928 touches  984 bots
    bot  929 touches  987 bots
    bot  930 touches  991 bots
    bot  931 touches  982 bots
    bot  932 touches  985 bots
    bot  933 touches  986 bots
    bot  934 touches  992 bots
    bot  935 touches  989 bots
    bot  936 touches  982 bots
    bot  937 touches  982 bots
    bot  938 touches  991 bots
    bot  939 touches  983 bots
    bot  940 touches  983 bots
    bot  941 touches  983 bots
    bot  942 touches  992 bots
    bot  943 touches  983 bots
    bot  944 touches  982 bots
    bot  945 touches  986 bots
    bot  946 touches  982 bots
    bot  947 touches  982 bots
    bot  948 touches  986 bots
    bot  949 touches  989 bots
    bot  950 touches  986 bots
    bot  951 touches  983 bots
    bot  952 touches  983 bots
    bot  953 touches  982 bots
    bot  954 touches  982 bots
    bot  955 touches  982 bots
    bot  956 touches   19 bots
    bot  957 touches  989 bots
    bot  958 touches  985 bots
    bot  959 touches  982 bots
    bot  960 touches  993 bots
    bot  961 touches  982 bots
    bot  962 touches  984 bots
    bot  963 touches  986 bots
    bot  964 touches  983 bots
    bot  965 touches  983 bots
    bot  966 touches  986 bots
    bot  967 touches  982 bots
    bot  968 touches  982 bots
    bot  969 touches  982 bots
    bot  970 touches  984 bots
    bot  971 touches   22 bots
    bot  972 touches  982 bots
    bot  973 touches  989 bots
    bot  974 touches  983 bots
    bot  975 touches  990 bots
    bot  976 touches  983 bots
    bot  977 touches  988 bots
    bot  978 touches  982 bots
    bot  979 touches  983 bots
    bot  980 touches  985 bots
    bot  981 touches  985 bots
    bot  982 touches  988 bots
    bot  983 touches  985 bots
    bot  984 touches  982 bots
    bot  985 touches  983 bots
    bot  986 touches  987 bots
    bot  987 touches  983 bots
    bot  988 touches  984 bots
    bot  989 touches  984 bots
    bot  990 touches  986 bots
    bot  991 touches  982 bots
    bot  992 touches  984 bots
    bot  993 touches  982 bots
    bot  994 touches  983 bots
    bot  995 touches  982 bots
    bot  996 touches  982 bots
    bot  997 touches  984 bots
    bot  998 touches  987 bots
    bot  999 touches  982 bots

I wonder how these densely connected ones relate: are any of them entirely
contained by the others? I guess they're not exactly identical or they would
show the same numbers.

The code I previously wrote, that pursues the 1000 most-covered regions,
eventually finds a region that overlaps with 982 bots which seems at least in
the right neighborhood.

I have yet to write code that actually finds the closest point to the origin in
that zone, but that should be possible.

    [src/bin/aoc23.rs:239] j = 982
    [src/bin/aoc23.rs:239] &cov[j] = {
        Zone {
            pxpypz: 82010405,
            pxpymz: 21511734,
            pxmypz: -280464,
            pxmymz: -60779126,
            mxpypz: 60779126,
            mxpymz: 280465,
            mxmypz: -21511732,
            mxmymz: -82010396,
        },
    }

So given a zone, what's the closest point in it to the origin?

    2x <= pxpypz + pxmymz
    2x <= pxmypz + pxpymz

    -x - y - z <= mxmymz
    -x + y + z <= mxpypz
    -2x <= mxmymz + mxpypz
    x >= (mxmymz + mxpypz) / 2



Taking this approach we get

    x <= 10615639
    x <= 10615635
    x >= 10615635
    x >= 10615633
    therefore x=10615635
    [src/bin/aoc23.rs:205] ymax1 = 41145434
    [src/bin/aoc23.rs:205] ymax2 = 41145430
    [src/bin/aoc23.rs:214] ymin1 = 41145430
    [src/bin/aoc23.rs:214] ymin2 = 41145429
    [src/bin/aoc23.rs:219] y = 41145430
    [src/bin/aoc23.rs:229] zmin1 = 30249331
    [src/bin/aoc23.rs:229] zmin2 = 30249331
    [src/bin/aoc23.rs:233] p = (
        10615635,
        41145430,
        30249331,
    )
    Solution to B: 10615635

But it says that solution (10615635) is too low.  Bah.

The interesting point about it being too low is that either this point doesn't
actually intersect the bots I think it does, or there's another one further out
that does touch more of them.

Let's check how many bots cover this point. I get the expected answer that it's
in range of 983 bots.

So if that answer is too low, there must be another point, further from the
origin, that is in range of more than 983 bots.

So one way to restate this is that we need to work out which bots can't reach,
and we know there are at most 16 of them. (Or, there's some bug.)

Almost all of the bots can reach this one point. However choosing 16 from 1000
bots would still be `~1000**16` which is pretty infeasible to do by checking
all combinations.

I wonder if this is a trick and there's any position that's in range of
everything? No, by the time we get to bot 45, there's no intersection left.
Not too surprising because we previously saw

    bot   45 touches  121 bots

This seems a bit inaccurate, but I wonder what would happen if we skip bots
that lead to an empty zone, and otherwise keep going?

Interestingly, that gives the same answer as before (and rather more quickly)

    [src/bin/aoc23.rs:390] included = 983
    [src/bin/aoc23.rs:390] zone = Zone {
        pxpypz: 82010405,
        pxpymz: 21511734,
        pxmypz: -280464,
        pxmymz: -60779126,
        mxpypz: 60779126,
        mxpymz: 280465,
        mxmypz: -21511732,
        mxmymz: -82010396,
    }

I wonder what would happen if we process the largest bots first. This is also
kind of a kludge though. That gives the same answer, interestingly.

...

So we could just guess points or try to bisect, but that doesn't give me much
confidence it'll find what this problem wants, which is the precisely right
solution.

Some different approach is needed.

Maybe we can do something with the fact that we don't ultimately need the full
list of points, just the closest Manhattan distance to the origin, and that's
also how the zones are defined.

...

OK, here's another take on this:

For a bot, we can easily determine the `x` coordinates that can possibly be in
range. Obviously not every `y` and `z` will be in range at that `x`, but for
that bot there will be some `y,z` for that `x`.

Using this, we can find some range of `x` that encompasses at least 983 bots,
and maybe the range that encompasses a larger range of bots.

...

Another thought about #23:

If three bots A, B, and C, all overlap with each other forming a clique, then
there must be one common region of overlap between all of them, because of the
convex shape of the diamonds covered by the bots. It's not possible for A to
touch B on the right, and A to touch C on its top, and B to reach up to C,
without the common region between B and C also overlapping with A.

Therefore, finding the point covered by the largest number of bots is the same
as finding the maximum clique of bots, and then finding the specific
intersection region between those bots, and then finding the points in that
region that are closest to the origin.

(Incidentally, on terminology, I discovered from Wikipedia that 'maximal'
clique is the largest clique including a specific node, whereas the 'maximum'
clique is the largest clique anywhere in the graph. It's a bit easily
confused.)

It's easy, and I already have code, to calculate an adjacency matrix between
bots.

However I was having trouble thinking of a scalable way to find the maximum
clique, and again from Wikipedia I confirmed that this is a classically NP-hard
problem. And `n=1000` here is pretty large, making any kind of combinatorial
approach infeasible.

So it seems there are a few options:

1. Thinking of it as a maximum clique isn't actually the right approach: there
   is a simpler way, perhaps working in the coordinate space or making use of
   the definition of distance.

2. Perhaps there's a documented but unobvious algorithm that can solve this in
   a plausible time even though it is NP-hard? That seems a bit unlikely. *The
   fastest algorithm known today is a refined version of this method by Robson
   (2001) which runs in time `O(20.249**n) = O(1.1888**n).`* So that's still
   1e75 steps, completely infeasible.

3. Perhaps although it's infeasible in the general case, it is soluble in the
   specific case of this input. It seems like there's one large chunky set of
   overlapping bots and perhaps only a few candidates really need to be
   considered for inclusion or exclusion.

From the data above, it seems there are many bots that overlap with ~983 other
bots. (I guess there are >983 of them?)  Perhaps there is even a higher number.

So perhaps this problem can be restated as: which of the ~17 poorly-connected
bots do we need to exclude, to still have a clique? There is probably no
question that the great majority of bots are in a single clique. For each of
them, if we find it doesn't overlap, we don't need to consider any other
possibilities for that bot.

So that means there are perhaps ~17 cases to test, or in the unlikely worst
case perhaps we need to check combinations of them and there are `~2**17`
combinations?

Sorting the adjacency count data above, this seems to become pretty clear:
there are 982 bots that touch >=982 bots, and after that it falls off quickly.
Around that break point:

    bot  960 touches  993 bots
    bot  879 touches  993 bots
    bot  795 touches  993 bots
    bot  662 touches  993 bots
    bot  634 touches  993 bots
    bot  600 touches  993 bots
    bot  593 touches  993 bots
    bot  549 touches  993 bots
    bot  400 touches  993 bots
    bot  377 touches  993 bots
    bot  339 touches  993 bots
    bot  324 touches  993 bots
    bot  319 touches  993 bots
    ... (many lines)
    bot   17 touches  982 bots
    bot   14 touches  982 bots
    bot    5 touches  982 bots
    bot    2 touches  982 bots
    bot  202 touches  523 bots
    bot  798 touches  361 bots
    bot  696 touches  313 bots
    bot   99 touches  271 bots
    bot  211 touches  207 bots
    bot  521 touches  191 bots
    bot  293 touches  166 bots

So it seems, pending verification:

1. There is one large clique of 982 bots. It's possible this is deceiving and
   they're not actually all well connected, in which case this might be hard
   again.

2. If there is a large clique of that size then bot 202 and others that come
   after it clearly cannot be included.

This hypothesis would be consistent with the earlier feedback from the oracle
that 975 bots is too low.

Am I saying I want to look for the largest `k` where there are at least `k`
bots that each overlap with `k` bots including themselves? Why is that
convincing?

Well, if there are not at least `k` bots with that many neighbors, clearly they
don't form a clique: the correct `k` can't be that high. On the other hand,
is there any guarantee that `k` is at least this high, and they do form a
clique? Couldn't it be that there are, let's say, two bots that each overlap
with just half of the popular bots, but it's hard to tell which?

That might be true.

If we say, `k` has to be at least close to this number, then perhaps it's easy
to see which of the bots in that list are weakly connected and can then cheaply
be excluded.

The question remains open, by the way, to work out how far the common region is
from the origin, but this should be just algebra and not inherently too hard.

...

So the next step here seems to be:

1. Make a list of the bots that touch >= 982 other bots.

2. Check they all actually touch 982 other bots.

3. Find the region that's common between all of them.

4. Find the closest point in that region to the origin.

...

I did find an intersection allegedly between all 983 bots:

    [src/bin/aoc23.rs:403] m = 983
    [src/bin/aoc23.rs:424] intersection_zone = Zone {
        pxpypz: 82010405,
        pxpymz: 21511734,
        pxmypz: -280464,
        pxmymz: -60779126,
        mxpypz: 60779126,
        mxpymz: 280465,
        mxmypz: -21511732,
        mxmymz: -82010396,
    }

Interesting that here I found 983 whereas previously I thought it would be 982.

Now assuming this is the most-intersected region, we'd need to find the
Manhattan distance of the closest point in it to the origin. At a guess I said
this would be the smallest absolute value of the constraints, although perhaps 
that's off-by-one given the inequalities that define the region.

I could also check that none of the other bots intersect this region at all...
OK, checked, and that's true.

Perhaps this is not the easiest representation of a `Zone` to work with...

OK, I already have a `closest_to_origin` function, and that seems to get an
answer that the site accepts. I'm still not totally happy that this will be
correct on all inputs, but it is right on this input, so that's good...

It turns out perhaps there is no really tidy answer:
<https://www.reddit.com/r/adventofcode/comments/aa9uvg/day_23_aoc_creators_logic/>.
