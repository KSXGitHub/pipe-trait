# Pipe Trait

[![Test](https://github.com/KSXGitHub/pipe-trait/workflows/Test/badge.svg)](https://github.com/KSXGitHub/pipe-trait/actions?query=workflow%3ATest)
[![Travis Build Status](https://travis-ci.org/KSXGitHub/pipe-trait.svg?branch=master)](https://travis-ci.org/KSXGitHub/pipe-trait)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=KSXGitHub/pipe-trait)](https://dependabot.com)

Add `pipe` method to every type.

## Usage Examples

### Same type

```rust
use pipe_trait::*;
let inc = |x| x + 1;
let double = |x| x + x;
let square = |x| x * x;
let a = (123i32).pipe(inc).pipe(double).pipe(square);
let b = square(double(inc(123i32)));
assert_eq!(a, b);
```

### Type transformation

```rust
use pipe_trait::*;
let x = 'x';
let a = x
    .pipe(|x| (x, x, x)) // (char, char, char)
    .pipe(|x| [x, x]) // [(char, char, char); 2]
    .pipe(|x| format!("{:?}", x)); // String
let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
assert_eq!(a, b);
```

## License

[MIT](https://git.io/JfgHW) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
