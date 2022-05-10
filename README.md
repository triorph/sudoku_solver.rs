# Sudoku Solver

I saw a somewhat scathing article recently about how one person tried to use TDD
to get a sudoku solver, while another person just thought about it for a bit and
got an "elegant" solution in less than 100 lines. The idea was that TDD does not
get you the answer, and you still actually have to think.

As a firm believer in tests, I want to take the other route and somewhat try to
prove this post wrong, except that I will still actually have to think so its
not really proving it wrong, I just want to show that TDD can still get the
result if you're willing to think along the way, and you'll get a better (and in
my opinion more elegant) solution at the end.

## The approach

Rough outline of the algorithm is:

- Reduce until you cannot reduce anymore
- Find smallest divergence point, and parallelise the output from here. Only
  accept the answer that's logically coherent. (I guess we need to handle
  multiple solutions and no solutions also?)

### Reducing

The easy way:

- if there's only 1 number available for a slot in its horizontal, vertical or
  block components, then assign that.

Slightly more difficult:

- For a given horizontal, vertical or block component, if a number can only
  be assigned in one place (but it still has multiple other possibilities) then
  this is the actual place it must go.

# To do

- I've done some minor optimisations, but I suspect the "rebuilding the reduce
  graph every time" could easily do with some caching.
- I have still somehow avoided doing async stuff in rust, despite it supposed to
  be giving you "Fearless concurrency". The splitting seems like a good place to
  try that.
