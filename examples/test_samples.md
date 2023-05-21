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
arr[1] = 10
arr[1] # 10
arr[-1] # Index of an array should not be less than ZERO.
arr[4] # Array reading out of range, expected index < 4, found 4.
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
Math.sin(1) # 0.84147098
Math.sin(1) ^ 2 + Math.cos(1) ^ 2 # 1
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
print = fn(i $Numb) {out i}
plus = fn(a $Numb, b $Numb) {a + b}
plus1 = fn(i $Numb) {i + 1}

recurse = fn(i $Numb) {if i == 5 {brk}; out i; recurse(i + 1)}

t = fn() {arr[index] = element}
```

# class definition

```calcrs
Person = cls {age $Numb, name $Str}
inst = new Person(10, "test")
nested_inst = new Person(10, new Person(1, 2))
```