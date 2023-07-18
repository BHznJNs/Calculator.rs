# Calculator.rs

这是一个用 Rust 编写的简单命令行计算器程序。

## 特性

- 支持变量

```text
甲 = 1
输出 甲 # 1
```

- 支持数学函数

```text
输出 数学.sin(1) # 0.84147098
```

- 支持惰性表达式

```text
甲 = 10
乙 = 20
求和 = { 甲 + 乙 }
输出 求和() # 30
甲 += 1
输出 求和() # 31
```

- 支持注释

```text
1 + 1 # 加法
```

- 支持数组

```text
测试数组 = 实例 数组类([1, 2, 3])
测试数组.尾插(4)
输出 测试数组._
# [
#   1, 2, 3, 4,
# ]
```

- 支持函数定义

```text
加一 = 函数(任意数) {中断 任意数 + 1}
输出 加一(1) # 2
```

- 支持脚本执行

```text
calculator.exe script.calcrs
```

- 支持面向对象编程

```text
人类 = 类 {年龄 $数字, 名字 $字符串}
人类实例 = 实例 人类(10, "测试名称") 
输出 人类实例
# {
#   年龄: 10
#   名字: "测试名称"
# }
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
