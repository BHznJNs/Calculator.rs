# Calculator.rs

An simple command-line calculator program writen with Rust.

## Features

- Variable support

```text
> a = 1
= 1
> a
= 1
```

- Math functions support

```text
> sin(1) 
= 0.84147098
```

- Lazy-Expression support

```text
> a = 10
> b = 20
> sum = {a + b}
> sum()
= 30
> a += 1
> sum()
= 31
```

- Comment support

```text
> 1 + 1 # plus
= 2
```

- Array support

```text
> arr = [1, 2, 3] 
= [
  1, 2, 3,
]
> push(arr, 4) 
> arr
= [
  1, 2, 3, 4,
]
>
```

- Function defining support

```text
> plus1 = fn(i $num) {i + 1} 
> plus1(1) 
= 2
```

- Script execute support

```text
calculator.exe script
```

- OOP support

```text
> Person = cls {age, name}
= {
  name,
  age,
}
> inst = new Person["test", 10] 
= {
  name : 'test',
  age : 10,
}
```

## Installation

### Windows

Go to the [release page](https://github.com/BHznJNs/Calculator.rs/releases) and download the latest released .exe file

Then just execuse it.

### Other OS

You need to install the Rust (version >= 1.60).

Clone this repository

```sh
git clone https://github.com/BHznJNs/Calculator.rs
cd Calculator.rs
```

Then, use Cargo to compile

```sh
cargo build --release
```

The release executable file will be at: `Calculator.rs/target/release`

## Tutorial

See here: [Syntax Tutorial](./examples/syntax)

## Build-ins

- Functions
  - Basic
    - input
    - type
    - clone
    - int
    - float
    - string
    - ascii

  - Array
    - push
    - pop
    - shift
    - unshift
    - insert
    - remove
    - len

  - Math
    - sin
    - cos
    - tan
    - asin
    - acos
    - atan
    - sinh
    - cosh
    - tanh

    - rad
    - deg

    - log10
    - log2
    - ln
    - exp

    - abs
    - sqrt
    - floor
    - round

- Constants
  - PI
  - E

  - VOID
  - NUM
  - STR
  - ARR
  - LEXPR
  - FUNC
  - CLS
  - OBJ