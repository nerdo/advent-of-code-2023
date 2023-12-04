#

I should have started documenting this kind of information sooner.

I love Advent of Code.

I think it's a great way to hone skills, find your strengths and weaknesses, and improve on it all.

I've been partially using it as an excuse to practice with Rust, and for sure, I'm rusty in the worst sense of the word lol but it's still been a great experience.

Today was rough though.

I set out to try and conquer the exercise in about an hour. For part 1, I did have a solution that worked against my test case in around that time, but it took me several more hours to actually figure out why I was getting the wrong number.

## Part 1: Where I went wrong

tldr: I created a bug with reading numbers that are on the edge of the grid.

Consider a grid like this:

```
...%..45
678...*.
```

In this case, each one of those numbers should be considered a part number, and my code for detecting symbols near numbers was working great right out of the gate.

But I failed to notice that because of the way I pre-processed the grid and neglected to check for borders, I created a situation where instead of the number `45` being detected as a part number, I was getting `45678` instead.

I removed all the newlines and had a 1 dimensional buffer that I queried, but in the place where I was reading numbers, I used string slices and I figured, oh I can just find the beginning of a number and have it parse the string from there, but I needed to cap it to the end of the line and it took hours for me to realize that's where the error was.

### Lesson: Instead of getting tangled up by edge cases, try to eliminate them.

After a brief cruise of the reddit thread on this year's AoC, I came across a few spoilers that alluded to this approach, basically mentioning that if you add a border to the schematic (presumably with `.`'s), then you don't have to deal with edge cases.

That would have saved me, and honestly, I would have probably gotten it right in one shot because there were no other errors in the logic that I needed to fix.

## Part 2: Where I went wrong

I actually got part 2 on the first attempt, but it took me a lot more code refactoring than I wanted to get there.

Part of the problem is that I didn't sufficiently break the code into domain layers. I had a monolithic function that was handling parsing the data, accessing it, and producing the part numbers.

In Part 2 I took a better approach. I separated those layers into functions, started high level, and drilled down until I had no more `todo!()`'s left to implement.

### Lesson: Spend more effort on separating concerns.

I've been a developer for a long time, but I have always struggled with organizing code well.

With TDD, I've gotten better, but clearly have some gaps.

I think a good approach next time will be to focus some energy on making sure the first layer defines WHAT the code intends to do, and subsequent layers focus on HOW it does it.
