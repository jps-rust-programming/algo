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

- right = 0, s[right] = 'a', not in seen, update seen = {'a': 0}. max_len = max(0, 1 - 0 + 1) = 1.
- right = 1, s[right] = 'b', not in seen, update seen = {'a': 0, 'b': 1}. max_len = max(1, 2 - 0 + 1) = 2.
- right = 2, s[right] = 'c', not in seen, update seen = {'a': 0, 'b': 1, 'c': 2}. max_len = max(2, 3 - 0 + 1) = 3.
- right = 3, s[right] = 'a', already in seen at index 0. Move left = 1. Update seen = {'a': 3, 'b': 1, 'c': 2}.
- right = 4, s[right] = 'b', already in seen at index 1. Move left = 2. Update seen = {'a': 3, 'b': 4, 'c': 2}.
- right = 5, s[right] = 'c', already in seen at index 2. Move left = 3. Update seen = {'a': 3, 'b': 4, 'c': 5}.
- right = 6, s[right] = 'b', already in seen at index 4. Move left = 5. Update seen = {'a': 3, 'b': 6, 'c': 5}.
