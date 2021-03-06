Generational Indices
=========================================================================

-   [Intro](#intro)
-   [Design](#design)
    -   [Index and Generation numbers](#index-and-generation-numbers)
-   [Examples](#examples)

Intro
-----

This is a crate under the rubric of \"do one thing very well\". We
implement the \"bare bones\" generational index here, without all the
trappings. You are free to use this to implement your own Entity
Conpoment System / Data Driven application as you see fit.

Later on, we will provide something that will implement something for
vectors at a later date.

Design
------

In Rust, especially in game development, but in other areas, it is
better to use a entity approach rather than the normal object-oriented
approaches, if for no other reason that it makes the Borrow Checker
happy.

In addition, it will be easier to optimize for caches to keep your data
structures near each other so that they can wind up in your L2 or L3
caches.

For this reason, Gen-Indices exist.

### Index and Generation numbers

Basically, when you grab a new index, it comes with an initial
generation number of zero. When you delete that index, it is cached in a
delete queue. When a new index is requested, it will first check the
delete queue, and if an entry is present, it is given to you instead,
with the genertion number incremented.

The new index is always monitically increasing; it shall never be reset.
The same is true with the generation numbers.

Examples
--------

``` {.rust}
extern crate gen_indices;
extern crate num;

use gen_indices::*;
use num::{Num, zero, one};

let gi = GenIndexEntitySet::<u64, u64>::new();

// first index
let idx1 = gi.lock().unwrap().next_index();
println!("first: {:?}", idx1);

// second index
let idx2 = gi.lock().unwrap().next_index();
println!("first: {:?}", idx2);

// delete first index and then get next index
if let Err(e) = gi.lock().unwrap().delete_index(idx1) {
  println!("Error: {}", e);
}
let idx3 = gi.lock().unwrap().next_index();
println!("first: {:?}", idx3);
```
