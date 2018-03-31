# Sed Notes

This project aims to create a stream editor similar to `sed`. As a result, it
is worth reviewing some notes about `sed` taken from its documentation. This
document will cover some details about how `sed` works and how it is used, so
that we have something to compare this project to, and to understand what
components we will need ourselves.

### `sed` Invocation Syntax

The definition of `sed`'s invocation syntax is found at the top of its man
page, and looks like this.

```
sed [OPTION] ... {script-only-if-no-other-script} [input-file] ...
```

This means that in the arguments, we will expect 0 or more options first. If
the `-e` or `--expression` flag is not found, then we will expect a script
enclosed within a set of `{` and `}` characters. Finally, we will expect
0 or more arguments specifying paths to input files, otherwise we will read
from the standard input.

### How `sed` Works

Sed maintains two buffers, each initially empty, called the 'active pattern
space', and the 'auxiliary hold space'. Sed reads one line at a time from the
input stream, removes the trailing newline, and places it into the pattern
space buffer.

Once a line has been placed into the pattern space, the stream editing commands
are evaluated. Each command can have a condition associated with it, such as
line numbers to perform an operation on. If the conditions for a command are
satisfied, then the operation is performed. Once each command has been
evaluated on the line, the trailing newline is replaced and the result is sent
to the output buffer.

### Commands, Addresses

Let's review some common `sed` commands, their syntax, and their meaning. Note
that these are not the same as the option flags to give `sed` when running it,
these are the editing commands themselves.

*  `d` - delete the pattern space, move to the next cycle.
*  'p' - print the pattern space. (used with the -n flag, which disables automatic printing.)
*  'i' - prefix the line with some text, with a newline.
*  'a' - append the line with some text, with a newline.
*  's' - search for text and replace it with a new value.

One way that commands can be categorized is by the address conditions that they
accept. Certain commands accept 0 addresses, 0-1 addresses, and others accept
an optional address range. Address conditions can be specified in a number of
ways, but these are the most common methods in my experience:

*  `number` - Only match the given line nuber.
*  `first`~`step` - Starting at `first`, match every `step`th line.
*  `a`,`b` - match the lines from `a` to `b`.
*  `a`,+`b` - match lines from `a` to `a + b`.

### Option Flags

There are some important option flags that are used when invoking `sed`, these
play a role in passing our commands to the program, and for adjusting some
i/o behavior.

*  `-e`, or `--expression` - Add an expression to the list of commands to run.
*  `-f`, or `--file` - Add the contents of a file to the list of commands to run.
*  `-n` or `--quiet`, or `--silent` - Disable automatic printing to the output.

There are other options that can sometimes prove to be useful, but these are
the most common options used when using `sed` for most stream editing tasks.
