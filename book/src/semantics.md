# Semantics

Everything in Table is deep clone by default, this can and is highly inefficient, but can be averted with the use of the `@` character.
This makes whatever value actually pass the reference to said value. I chose `@` instead of `&` cause `@` makes more sense to newbies.
It's the value *at* that location.

If you already have an `@` and you wanna clone it, should use the `*` aka the dereference operator.


Some semantics just layed out in code
```

let x1 = 10;
let x2 = 1.0;
let my_thing = ["yo", x1, a: @x2, 1];
my_thing[0] == "yo"
my_thing[1]++;
print(x1) // "10"
x2 += 0.2;
my_thing.a // "1.2"
my_thing[2] == my_thing.a
my_thing[1] != x1
my_thing.a += 1.0;
print(x2) // "2.2"

let [a, b, c] = some_function(); //returns a table with 3 elements in it.
let [this_thing: other_name, 2: here_now, 1: now_here] = other_function();

```

```

let x1 = 10;
let x2 = 1.0;
let x3 = "yo";

let some_table = [val1: x1, val2: @x2, val3: @x3];

let [@x2, @x1, x3] = some_function();

some_table.val1 // "10"
some_table.val2 // "42069"
some_table.val3 // "yo"

x1 // "420"
x2 // "42069"
x3 // "hey there partner"
```