# Part 1

## Impressions

This seems like it might be something that could be easily solvable with a graph... Like, surrealdb.

If I parse the data into a graph, I can then query it to find the answer.

I could also just create maps of each relationship, but this may be simpler with surrealdb - and a good excuse to use it!

## Problems

### Attempt 1

Builder pattern: https://www.youtube.com/watch?v=Z_3WOSiYYFY

I should have read the directions more thoroughly!

I thought I understood the ask but I missed the part about how any source numbers that aren't mapped correspond to the same destination number.

I also didn't fully read through the walkthrough of the solution and i botched it up, going down a rabbit hole and tinkering with surrealdb, which was a good exercise if it wasn't a complete waste of time for this challenge.

...back to the drawing board!

### Attempt 2

Ok, so I've removed surrealdb and the code I wrote with it, and went for a straightforward approach, but that didn't work.

It worked for the test just fine, but not for the real thing, and I should have known.

I should have tested my solution "on paper" against the real input before I started coding.

I took the approach of expanding the maps in memory. In other words, I created hash maps where I processed the count and created an entry for every single possible value.

This doesn't work because it takes too long... I _might_ be able to run it on my laptop with 32GB RAM or my PC with 64GB RAM if I'm lucky, but the approach sucks...

Back to the drawing board again!

But this time, with a plan:

1. Before writing code, test the theory "on paper"...
2. When the theory is sound, refactor the code to process the maps without expanding them fully.

### Attempt 3

Huzzah!

It worked.

# Part 2

## Impressions

A brute force approach here won't work. Similar to Part 1, if one were to take a straightforward approach of mapping through every possible input value, memory (and time/CPU cycles) would be an impediment.

The first part of the problem forced me to deal with compressed mapping data; the second part seems to suggest/force dealing with compressing input data.

It seems like the right approach here is to refactor the code so that the input becomes ranges of numbers and the output produces ranges of locations.

At that point, it should be trivial to select the lowest location number by simply sorting the resulting list of location ranges in ascending order and choosing the first one.
