# aoc20

The problem seems to fall into two parts:

1. Generate a map by following every possible path.

2. Number all the squares by their shortest-path to the starting point, and
   return the maximum shortest path found.

The regex instructions can contain branches and nesting, where the branches
might be empty.

To walk it exhaustively, in theory, we need to generate every possible path that
will match the expression. It might be possible to do something cheaper, if
we work out that two branches of the same alternative lead us back to the
same point.

Number squares with (0,0) in the top-right, for easiest printing.

## Map representation

For the map, it seems that all we need to know is, which squares have a door to
the south, and which have a door to the east. This captures everything we need
to know:

    door_s[(x, y)] implies
      (x, y) is a room
      (x, y+1) is a room
      you can travel between them in either direction

    similarly for door_e

Given this we can update the map by noting that you can travel in any direction
from `p`, and use that to update the door matrices. We can also return all
neighbors of a given room.

Let's put the origin at 0, and allow for negative coordinates.

## Regex traversal

There are three cases that occur in the regexp:

1. A sequence of steps: `ABC`

2. Alternation: `Y(A|B)Z`

3. Nesting: `Y(A(B|C)|D)Z`

Naively we can handle alternation and nesting by recursion, although if the
nesting is very deep perhaps Rust's own stack can't accomodate it.

Sequencing is simple: step in the next direction, implicitly remembering there
is a door there.

Alternation requires following to completion all of each branch, and then
everything that comes after it in the regexp, then returning to the current
point to process the next branch.

We seem to need some kind of stack remembering branches not yet completed, that
we have to come back and evaluate.

It might be nice to just handle parens and alternatives in a pass through the
string.

Keep a stack of indexes into the instructions of open parens, and the turtle
position when they were encountered. Also keep with that paren, a list of
positions needing to be continued. Then proceed onwards into the contents of
the first alternative inside that paren.

Basically, we keep a set of turtles walking through the maze. They multiply
every time we reach an alternative. Every turtle needs to know:

* Its current location

* The stack of locations where it last saw an open paren (because new clones
  will start there.)

On reaching `|`:

* The current branch concludes, by remembering its ending turtle position needs
  to be picked up and continued on reaching the next close paren (if it's not
  already in that list.)

* The next branch begins, first setting its turtle position to the start of this
  alternative.

On reaching `)`:

* As before, the current branch (the last branch in the alternate) concludes.
  In the minimal case, there was only one branch and it was empty, so it pushes
  the start position of the alternate into the list of branches to pick up.

* Pop off the start paren, whose position is no longer needed, and the list of
  turtle positions to pick up. Each of them now resumes interpreting
  instructions from the character after the closing paren.

I wonder if it's easier to precalculate the offsets of matched parens so that we
can skip directly to the end?

We could recast the regexp as being a sequence of alternates, treating the
trivial case of top-level letters as just being a single alternate with a single
branch.

Then

    R         := TERM*
    TERM      := LITERAL | ALTERNATE
    LITERAL   := LETTER*
    LETTER    := 'N' | 'S' | 'E' | 'W'
    ALTERNATE := '(' ALTC? ')'
    ALTC      := (TERM '|')* TERM

So again walking through this we need to multiply N turtles at each N-way
alternate. Each of them evaluates its contents left to right.

What if anything is the advantage of this? Well, it separates the work of
finding parens, from walking through it. I'm not sure that's worthwhile.

...

OK, suppose we do pre-parse all the alternatives into some kind of list so as to
avoid the complications of traversing the instruction characters at the same
time as the logical program.

Suppose also we wanted to just run through the _first_ of all possible
alternatives, without worrying about multiplying out turtles, for now.

## Depth calculation

It would be nice to fuse path depth calculation with building of routes.

It seems the path depth on which we first visit any given square, is not
necessarily the shortest path to it.

In particular it's possible we build a door from p1 to p2, and then later
build a shorter path to p1. We'd need to know to update the shortest-depth
on p2 too.

So, every time we visit a square:

*  If this square was never visited before, it will have no depth recorded,
   and we have just opened the first door to it. Its depth is that of the
   neighbor from which we just entered, plus one.

*  If the current path to it is longer than previously known, no further
   action is required.

*  Otherwise, we have visited it before but have now found a shorter path,
   and we need to update all its neighbors.

Because the updates of neighbors requires tracing paths that wouldn't need
to be follow just for building doors, it seems easier to do this in a second
pass after building the map.
