# Check prime number

- `Basic Loop`: Checks for divisibility from `2 to n-1`.
- `Efficient Sqrt Method`: Checks divisibility only up to `sqrt(n)`.
- `6k ± 1 Optimization`: A more efficient method that skips obvious `non-prime` numbers.
- `Recursive Method`: Uses recursion to check divisibility.
- `Sieve of Eratosthenes`: Efficient for checking all primes in a `range`.

## longest substring without repeating characters

The most efficient way to solve this problem is using the `sliding window technique` combined with a `hash set` or `hash map` (dictionary) to track the characters and their indices.u
u

### Example Walkthrough:

- `Input`: "abcabcbb"

- `Initialize`: left = 0, max_len = 0, seen = {}.

**Step-by-step:**

- right = 0, s[right] = 'a', not in seen, `update seen` = {'a': 0}. max_len = max(0, 1 - 0 + 1) = 1.
- right = 1, s[right] = 'b', not in seen, `update seen` = {'a': 0, 'b': 1}. max_len = max(1, 2 - 0 + 1) = 2.
- right = 2, s[right] = 'c', not in seen, `update seen` = {'a': 0, 'b': 1, 'c': 2}. max_len = max(2, 3 - 0 + 1) = 3.
- right = 3, s[right] = 'a', already in seen at index 0. Move left = 1. `Update seen` = {'a': 3, 'b': 1, 'c': 2}.
- right = 4, s[right] = 'b', already in seen at index 1. Move left = 2. `Update seen` = {'a': 3, 'b': 4, 'c': 2}.
- right = 5, s[right] = 'c', already in seen at index 2. Move left = 3. `Update seen` = {'a': 3, 'b': 4, 'c': 5}.
- right = 6, s[right] = 'b', already in seen at index 4. Move left = 5. `Update seen` = {'a': 3, 'b': 6, 'c': 5}.

**Key Points:**

- `Ownership and Borrowing`: Understand the concepts of `ownership` and borrowing in Rust. When a value is moved, its ownership is transferred, and you can no longer use it in its original scope.
- `Lifetime Annotations`: In more complex scenarios, you might need to use `lifetime annotations` to ensure that references are valid.
- `Cloning`: Cloning a `HashMap` can be expensive, especially for large HashMaps. Use it judiciously.

## Strong code analysis skills in rust

Developing strong code analysis skills in Rust is essential for writing efficient, maintainable, and safe Rust programs. Rust is a systems programming language designed with safety and performance in mind. As such, it has unique features (like ownership, borrowing, and lifetimes) that need careful consideration when analyzing code.

## Key Areas to Focus On

### Understanding the Problem:

Like in any language, the first step is to understand the problem the code is solving. Focus on the problem statement, expected input and output, and the goal of the code.

#### Code Structure and Design:

Modularity: Is the code divided into smaller, reusable functions or modules? Rust encourages modularity through functions, structs, and enums. The use of these constructs can often make the code more readable and testable.
Data Flow: Trace how data moves through the code. For example, how are variables passed between functions? In Rust, ownership plays a huge role in data movement and needs to be carefully tracked.
Rust's Ownership, Borrowing, and Lifetimes:

Ownership: Rust's ownership system ensures memory safety without a garbage collector. Every piece of data has one owner, and when ownership is transferred, the original owner can no longer access that data.
Borrowing: Rust allows borrowing of data via references. There are two types of borrowing: immutable (&T) and mutable (&mut T), and borrowing must adhere to strict rules.
Lifetimes: Lifetimes specify how long references are valid and prevent dangling references. Code analysis in Rust often requires ensuring that lifetimes are correctly annotated to avoid issues.
Memory Safety and Performance:

Memory Leaks and Unsafe Code: While Rust ensures memory safety through ownership, borrowing, and lifetimes, it allows developers to use unsafe blocks for low-level memory manipulation. Analyze code for potential risks when using unsafe.
Zero-Cost Abstractions: Rust emphasizes zero-cost abstractions, where higher-level abstractions should not introduce overhead. It’s essential to understand the performance implications of abstractions like iterators or closures.
Concurrency:

Rust has a strong focus on safe concurrency. You should analyze code for the correct use of Rust's concurrency tools (e.g., Arc, Mutex, RwLock, async/await) and ensure that data races or other concurrency issues are avoided.
Error Handling:

Rust uses Result and Option types for error handling instead of exceptions, which encourages explicit handling of errors. Code should be analyzed to ensure errors are handled in a way that guarantees the program’s robustness.
Testing:

Rust's built-in test framework makes it easy to write unit tests. Ensuring that the code has adequate test coverage is a key aspect of code analysis. Look for proper test cases and edge cases in the code.
Code Readability and Style:

Rust has a rich set of idioms and patterns, such as Pattern Matching, Rust Iterators, Error Propagation with ?, and Result and Option chaining. It's essential to ensure the code adheres to the Rust style guide and is readable for others.
