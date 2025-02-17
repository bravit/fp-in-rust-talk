# Functional Programming For The Working Rustacean

*Vitaly Bragilevsky at RustNation UK*

_February 20th, 2025_

## Coding in a _functional style_

* Immutability: 
  * `input -> output`
  * ~~`data.mutate()`~~
  * works great combined with ownership passing
* Higher-order functions:
  * `map(|e| ...)`, `filter(|e| ...bool...)`, `reduce(|acc, e| ...)`
  * closures `|  |{  }`
  * passing functions as arguments
  * storing functions in variables and collections
  * returning functions
* Pure functions and effect management (sorry, no)
* Constructing datatypes with `struct`/`enum`
* Pattern matching

```








```

## The spectrum of coding in a functional style

1. Example: [Reading lists from a file](examples/reading_lists.rs)
2. Example: [Meals and menus](examples/meals_and_menus.rs)
3. Example: [Processing antennas (Do we really want this?)](examples/processing_antennas.rs)
4. Example: [Validation logic with `frunk`](examples/validate.rs)

```











```

## Reasonable functional programming in Rust

* Our goals:
  * simplicity
  * readability
  * correctness
* Non-goals:
  * applying new language features
  * pushing the limits of a type system (google for `higher-kinded types rust`)
* Other important considerations
  * Do we lose performance while being functional?
  * Hiding excessive memory: `std` vs `itertools`


```








```

## Links

* [Gilad Bracha, Deconstructing Functional Programming](https://www.infoq.com/presentations/functional-pros-cons/)
* [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html#functional-language-features-iterators-and-closures) (The Rust Book)
* [Adam Warski, What is Functional Programming?](https://softwaremill.com/what-is-functional-programming/)
* [Nikolay Yakimov, Rust is Not a Functional Language](https://serokell.io/blog/rust-is-not-a-functional-language)
* [Rust Iterators Beyond the Basics](https://blog.jetbrains.com/rust/2024/03/12/rust-iterators-beyond-the-basics-part-i-building-blocks/)