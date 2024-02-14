# Rustling exercises

Solved [Rustling](https://github.com/rust-lang/rustlings) exercises.
- Version used: 5.6.1

This repository and future ones will serve as a documentation of my Rust learning path. 

There is no better way to solidify one's understanding of a subject than by explaining it to others or recounting one's own perspective. 
The paragraphs below aim to share my insights concerning certain parallels and advancements from C++ in the context of Rust. 
It’s crucial to note, though, that these reflections are from a beginner’s standpoint, and their application may be limited in clarity or precision.

# C++ vs Rust

## Similarities 

### - Syntax structure.
Rust's syntax closely resembles that of C++, as both are [statically and strongly typed languages](https://www.techtarget.com/searchapparchitecture/tip/Static-vs-dynamic-typing-The-details-and-differences). Many keywords and basic types are similar between the two. Rust is expressive, and some argue it is easier to read (though I may reserve judgment on that until I become more familiar with it).

### - Some similar concepts

- Borrowing and 'std::unique_ptr'
In both C++ and Rust, the concept of borrowing plays a crucial role in managing ownership and references. 
In C++, std::unique_ptr represents exclusive ownership and ensures that a resource is deallocated 
when the owning pointer goes out of scope. 

Similarly, Rust’s borrowing mechanism allows for safe references without sacrificing ownership, offering parallels 
to the functionality provided by std::unique_ptr


- Rc<T> and 'std::shared_ptr'

Another notable convergence between C++ and Rust lies in 
their handling of shared ownership. C++ employs std::shared_ptr, 
a smart pointer that enables multiple pointers to share ownership
of the same resource while automatically managing its lifetime. 

In Rust, the Rc<T> type serves a similar purpose, allowing multiple references to the same data with reference counting to track the shared ownership. This similarity underscores the importance of managing shared resources efficiently,
whether in the context of C++ or Rust.

## Upgrades from C++
- Compile-time memory checks

One notable advancement in Rust compared to C++ is its approach to memory management,
which is predominantly done at compile time. Rust’s ownership system and borrow checker
work in tandem during compilation, ensuring memory safety without the need for a garbage 
collector at runtime. This stands in contrast to C++, where memory management often involves runtime mechanisms, 
such as manual memory deallocation or reliance on smart pointers.
Thus the compiler is permitting us from passing dangling references.

- Templates/concept features
Rust retains the concept of templates, a feature familiar to C++ developers. 
However, Rust goes a step further by having concepts enabled by default. 
In C++, concepts are a C++20 addition that allows developers to constrain templates, 
specifying requirements on template parameters. In Rust, this concept is integral, 
providing enhanced type safety and abstraction capabilities by default. 
This streamlined approach can lead to clearer and more concise code, compared to the more explicit syntax often required in C++.

## Exercise to Book Chapter mapping (original content in the Rustling Readme)

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| variables              | §3.1                |
| functions              | §3.3                |
| if                     | §3.5                |
| primitive_types        | §3.2, §4.3          |
| vecs                   | §8.1                |
| move_semantics         | §4.1-2              |
| structs                | §5.1, §5.3          |
| enums                  | §6, §18.3           |
| strings                | §8.2                |
| modules                | §7                  |
| hashmaps               | §8.3                |
| options                | §10.1               |
| error_handling         | §9                  |
| generics               | §10                 |
| traits                 | §10.2               |
| tests                  | §11.1               |
| lifetimes              | §10.3               |
| iterators              | §13.2-4             |
| threads                | §16.1-3             |
| smart_pointers         | §15, §16.3          |
| macros                 | §19.6               |
| clippy                 | §21.4               |
| conversions            | n/a                 |
