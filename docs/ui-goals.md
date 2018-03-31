# Pupset UI Goals

This document intends to lay out some goals for the command-line interface
exposed by the `pupset` program. This will identify what the differences
from `sed`'s syntax will be, and hopefully explain why these choices were
made.

### Humanly-Readable Command Names

Complex logic can become hard to maintain using `sed`, because many of the
commands are single characters. Beyond simple search/replace tasks, it
can be difficult to understand what is happening inside of a nontrivial `sed`
script.

In light of this, I intend to design `pupset` so that the commands used are
named (optionally) using full words, with clear parameters. For example,
assuming we have a file containing a sequence of numbers 1-10 (each on an
individual line, starting at 1, upper-bound inclusive), I would like for these
commands to be synonymous:

```
$: sed -e "1,8d" foo.txt
9
10

$: pupset "delete:[1,8]" -- foo.txt
```

NOTE: This is still -very much- subject to change, I will need to formalize
what the invocation syntax for the program will be at this point. This is
only intended to emphasize a focus on human-readability when specifying
stream editing commands.
