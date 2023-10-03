![Calculator.rs Logo](./logo.svg)

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

- 支持分数

```text
> f1 = fraction(1, 2)
= (1 / 2)
> f1 + 1
= (3 / 2)
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

## 快捷键

| 按键 | 功能 |
| --- | --- |
| Ctrl + s | 开启 / 关闭 文件保存组件 |
| Enter (当文件保存组件启用时) | 写入文件 |
| Ctrl + g | 开启 / 关闭 定位组件 |
| Enter (当定位组件启用时) | 跳转到指定位置 |
| Ctrl + f | 开启 / 关闭 查询组件 |
| Enter (当查询组件启用时) | 跳转到下一个匹配的文本位置 |
| Shift + Enter (当查询组件启用时) | 跳转到上一个匹配的文本位置 |
| Ctrl + r | 开启 / 关闭 文本替换组件 |
| Enter (当文本替换组件启用时) | 切换文本替换组件到替换模式 (默认为查找模式) |
| Ctrl + n (当文本替换组件启用并且处于替换模式时) | 跳转到下一个匹配的文本位置 |
| Ctrl + s (当文本替换组件启用并且处于替换模式时) | 替换单个匹配的文本 |
| Ctrl + a (当文本替换组件启用并且处于替换模式时) | 替换全部匹配的文本 |
| Ctrl + z | 撤销 |
| Ctrl + y | 恢复 |
| Ctrl + e | 运行脚本 |
| Tab (当光标位置介于缩进与代码之间时) | 添加缩进 |
| BackTab (Shift + Tab) | 消除缩进 |
| Esc | 恢复编辑模式 (当处于非编辑模式) / 退出程序 (当处于编辑模式) |

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

请参阅此处: [教程](./tutorials/CN)
