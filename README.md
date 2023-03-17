# Calculator.rs

An simple command-line calculator program writen with Rust.

## Features

- Math functions support

```text
> sin(1) 
= 0.84147098
```

- Variable support

```text
> a = 1
= 1
> a
= 1
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

- Script execute support

```text
calculator.exe script
```

## Installation

Go to the [release page](https://github.com/BHznJNs/Calculator.rs/releases) and download the latest released .exe file

Then just execuse it.

## Build-ins

- Functions
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