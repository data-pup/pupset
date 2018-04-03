# Command Design Notes

My first step for working on conditions was getting some basic scaffolding
in place, before I thought further about the specific design layout for these
classes. Once this was complete, I decided that it would now be best to start
considering the specific interfaces that should be exposed in order to
construct an editing pipeline.

I will be generally structuring my program such that:
*  A list of command arguments given to the program will be converted into an
    ordered sequence (a pipeline) of commands.
*  Each line of input will be sent through the pipeline, and the resulting
    output will be passed to an output buffer.

The raw command line arguments to the program are divided into two general
sections: Program options, and command arguments. Program options relate to
configuration settings for the overall program, i.e. silencing automatic
printing, verbosity, etc. Command arguments refer to the arguments used to
build a command pipeline that will process the input itself.

### Command Structure

A command shall consist of the following components:
*  A command token - Specifies the type of command.
*  An address condition (optional) - Specifies line numbers to operate upon.
*  A filter condition (optional) - Specifies conditions to satisfy before processing.
*  A parameter list (optional) - Specifies command parameters, if needed.

As a line is passed through the command pipeline, it will be considered by
each command in the chain. If the line number satisfies the address condition,
and the filter condition (if either are given), then the corresponding action
will be performed upon the contents of the line, and passed to the next command
in the pipeline.

A command should be constructed using an array of one or more arguments from the
command argument section of the program input, including at least its command
token.

### Address Conditions

This is one section wherein I am deciding to deviate from `sed`'s design.
Roughly, my addressing scheme will be a subset of `sed`'s functionality, but
I would also like to use a different scheme for specifying addresses.

Addresses shall be enclosed in sets of `(`, `)`, `[`, and `]` characters,
denoting range inclusivity and exclusivity. A single address should be
enclosed in `[` and `]` characters, ranges may use `(` and `)` characters if
an upper or lower bounds is not inclusive.

```
[2]    // Line 2.
(1..3) // Line 2.
[0..3) // Lines 0, 1, 2.
[0..3] // Lines 0, 1, 2, and 3.
(1)    // INVALID. Enclose single addresses in `[` and `]`.
```

## Command Actions

After implementing an address condition submodule, and parsing logic to create
these conditions using a string argument given in the program arguments, I
needed to implement the actions associated with a command.

While function pointers would be one way to implement this in other languages,
this seemed like a problem that would be best solved with the trait system,
assuming I structured things properly. This would likely mean that the
`Command` struct that I had added previously would need to be refactored into
a trait. Then, the individual commands could implement this trait.

```rust
pub trait Command {
    fn new(args: Vec<String>) -> Self;
    fn should_run(&self, curr_line: Address) -> bool;
    fn run(&self, line: String) -> String;
}
```

This is what I expected the command trait to look like when I started this
refactoring work, but this is of course subject to change.
