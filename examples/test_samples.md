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