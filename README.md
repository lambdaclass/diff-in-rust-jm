# Diff in Rust

A small program to find the differences (line by line) between two files.

## Running the program

`cargo run file1 file2`

## A small example

file1 contents:
```
Hello,
World!
```

file2 contents:
```
Good Morning,
World!
```

When `cargo run file1 file1` is executed, the output is:
```
<Hello,
>Good Morning,
World!
```

Here the `<` symbol shows which lines belong only to the left file (file1), `>` shows the lines that are only on file2. If no symbol is in front of the printed line, then it's present on both files.
