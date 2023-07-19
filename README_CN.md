# Calculator.rs

[English](./README.md) | 简体中文

这是一个用 Rust 编写的简单命令行计算器程序。

## 特性

- 支持变量

```text
> a = 1
= 1
> a
= 1
```

- 支持数学函数

```text
> Math.sin(1) 
= 0.84147098
```

- 支持惰性表达式

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

- 支持注释

```text
> 1 + 1 # plus
= 2
```

- 支持数组

```text
> arr = new Array([1, 2, 3])
= {
  v: [
    1, 2, 3, 
  ]
  ...
}
> arr.push(4)
> arr
= {
  v: [
    1, 2, 3, 4,
  ]
  ...
}
```

- 支持函数定义

```text
> plus1 = fn(i $Numb) {i + 1} 
> plus1(1) 
= 2
```

- 支持脚本执行

```text
calculator.exe script
```

- 支持面向对象编程

```text
> Person = cl {age $Numb, name $Str}
= {
  age: Number
  name: String
}
> inst = new Person(10, "test") 
= {
  age: 10
  name: "test"
}
```

## 安装

### Windows || GNU/Linux || MacOS (x86_64)

请前往 [release 页面](https://github.com/BHznJNs/Calculator.rs/releases) 下载最新的可执行文件，

然后直接双击运行即可。

### 其他系统和平台架构

您需要安装 Rust (版本 >= 1.60)。

克隆此仓库:

```bash
git clone https://github.com/BHznJNs/Calculator.rs
cd Calculator.rs
```

然后，使用 Cargo 进行编译:

```bash
cargo build --release
```

发布的可执行文件将位于: `Calculator.rs/target/release`

## 教程

请参阅此处: [教程](./tutorials)
