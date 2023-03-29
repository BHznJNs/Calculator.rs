# Pure number calculation

```calcrs
1 + 1 # 2
1 + 2 * 3 # 7
1 + 2 * 3 ^ 2 # 19
1 + 2 * 3 ^ (2 + 2) # 163
```

# Variable

```calcrs
var1 = 1 # 1
var1 # 1
var2 = 2 # 2
var1 + var2 # 3
```

# Array

```calcrs
arr = [1, 2, 3, 4]
arr[0] # 1
arr[-1] # Index of an array should not be less than ZERO.
arr[4] # Index out of range.
```

# Lazy-expression

```calcrs
var = 1
plus = {var += 1}

plus() # 2
var    # 2
```

# Build-in functions

```calcrs
sin(1) # 0.84147098
sin(1) ^ 2 + cos(1) ^ 2 # 1
```

# for-loop & if statement

```calcrs
a = 0
condition = {if a == 5 {brk}}
for 10 {condition(); a += 1}
a # 5
```

# function definition

```calcrs
func = fn(i $num) {out i}
plus = fn(a $num, b $num) {a + b}
plus1 = fn(i $num) {i + 1}

recurse = fn(i $num) {if i == 5 {brk}; out i; recurse(i + 1)}
```