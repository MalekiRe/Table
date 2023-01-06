# Table
the table programming language

this just just a quick layout of the concepts in table.

There are strings, in the backend strings are all turned into an array of immutable bytes, but if you create or modify any strings at runtime then you are modifying a *table* of numbers(storing the chars) which make up the string.

For types actually present when programming, there is 
Number(f32),
Boolean(bool),
Nil,
Table(u32),

Tables are all heap allocated (currently) and on the stack they just point to a position on the heap. 

A table is an assosociative array, it is implemented very similar to how the Tables in lua were implemented in 5.0, that is, it is made up of 2 sections, a hashmap of strings to indicies, and a resizable array that you index into in O(1) time with indicies. 

Everything in table is either a Table, Number, Boolean or Nil, there are no other types, and all complex types are just table types.

Functions are 2 tables, one of numbers, which represents the bytecode the function will execute, and the other is the constants that the function will use.

Arbitrary functions can be compiled and called at runtime. 

Syntax is rather simple, there is the `if` `switch` `match` `while` `for` syntax. 

TODO complete this writeup lmfao
