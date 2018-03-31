# Sed Notes

This project aims to create a stream editor similar to `sed`. As a result, it
is worth reviewing some notes about `sed` taken from its documentation. This
document will cover some details about how `sed` works and how it is used, so
that we have something to compare this project to, and to understand what
components we will need ourselves.

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

### Commands

Let's review some common `sed` commands, their syntax, and their meaning. Note
that these are not the same as the option flags to give `sed` when running it,
these are the editing commands themselves.

*  `d` - delete the pattern space, move to the next cycle.
*  'p' - print the pattern space. (used with the -n flag, which disables automatic printing.)
*  'i' - prefix the line with some text, with a newline.
*  'a' - append the line with some text, with a newline.
*  's' - search for text and replace it with a new value.

One way that commands can be categorized is by the address conditions that they
accept. For example, there are
be affected by a command. For example, ``