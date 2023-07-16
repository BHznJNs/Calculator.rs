# Calculator.rs

English | [简体中文](./README_CN.md)

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
> Math.sin(1) 
= 0.84147098
```

- Lazy-Expression support

```text
> a = 10
= 10
> b = 20
= 20
> sum = {a + b}
= <Lazy-Expression>
> sum()
= 30
> a += 1
= 11
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
> plus1 = fn(i $Numb) {i + 1} 
> plus1(1) 
= 2
```

- Script execute support

```text
calculator.exe script
```

- OOP support

```text
> Person = cl {age $Numb, name $Str}
= {
  name,
  age,
}
> inst = new Person(10, "test") 
= {
  age: 10
  name: test
}
```

## Installation

### Windows || GNU/Linux || MacOS (x86_64)

Please go to the [release page](https://github.com/BHznJNs/Calculator.rs/releases) and download the latest released appropriate executable file,

Then just execuse it.

### Other OS && Platform

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

See here: [Tutorials](./tutorials)
