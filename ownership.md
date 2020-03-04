#### The Stack and the Heap

In many programming languages, you don’t have to think about the stack and the heap very often. 
But in a systems programming language like Rust, whether a value is on the stack or the heap has more of an effect on how the language behaves and why you have to make certain decisions. 
Parts of ownership will be described in relation to the stack and the heap later in this chapter, so here is a brief explanation in preparation.

Both the stack and the heap are parts of memory that are available to your code to use at runtime,
but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. 
This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, 
and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! 
Adding data is called pushing onto the stack, and removing data is called popping off the stack.

All data stored on the stack must have a known, fixed size. 
Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 
The heap is less organized: when you put data on the heap, you request a certain amount of space. 
The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. 
This process is called allocating on the heap and is sometimes abbreviated as just allocating. Pushing values onto the stack is not considered allocating. 
Because the pointer is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Think of being seated at a restaurant. When you enter, you state the number of people in your group, 
and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, 
they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; 
that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, 
because the operating system must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that managing heap data is why ownership exists can help explain why it works the way it does.




#### Ways Variables and Data Interact: Move

```rs

let x = 5;
let y = x;

We can probably guess what this is doing: “bind the value 5 to x; 
then make a copy of the value in x and bind it to y.”
We now have two variables, x and y, and both equal 5. This is indeed what is happening, 
because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

```

But at the **String** version:

```rs

let s1 = String::from("hello");
let s2 = s1;

This looks very similar to the previous code, so we might assume that the way it works would be the same: 
But this isn’t quite what happens.
A String is made up of three parts, shown on the left: 
a pointer to the memory that holds the contents of the string, a length, and a capacity. 
This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

```

![string version](https://doc.rust-lang.org/book/img/trpl04-01.svg)


