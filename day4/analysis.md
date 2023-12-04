# Part 1

So far, this puzzle is much simpler than previous ones.

I did run into an issue that caused me to not get it on the first try, but it was because I incorrectly implemented error handling in Rust.

In fact, the reason why I even bothered/explored Rust error handling without a 3rd party crate is because I felt comfortable and clear on the program flow and the different responsibilities of each part.

# Part 2

## First Impression

I haven't started on part 2 yet, but I have read the description and I think I laid the foundation to be able to refactor what I wrote to handle Part 2 without too much fuss.

It took me a while to understand the instructions, but it seems pretty simple now that I have digested it.

Seems like I should just need to add a new data structure, a `HashMap` keyed by the card id with values being how many of those cards were received. By incrementing the correct counters while scoring the cards, one should be able to just sum the values and get the answer... TBD!

## Analysis

That worked out pretty much exactly as I thought. Not a terribly difficult exercise today, although, some of the others weren't that difficult... they just tripped me up with some details that took me far too long to resolve.

I'm hoping I can refine my workflow so that I will be able to minimize the liklihood of getting tripped up.
