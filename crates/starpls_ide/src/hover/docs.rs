pub(crate) const IF_DOCS: &str = r#"
```python
if
```

An `if` statement evaluates an expression (the _condition_), then, if
the truth value of the condition is `True`, executes a list of
statements.

```text
IfStmt = 'if' Test ':' Suite {'elif' Test ':' Suite} ['else' ':' Suite] .
```

Example:

```python
if score >= 100:
    print("You win!")
    return
```

An `if` statement may have an `else` block defining a second list of
statements to be executed if the condition is false.

```python
if score >= 100:
        print("You win!")
        return
else:
        print("Keep trying...")
        continue
```

It is common for the `else` block to contain another `if` statement.
To avoid increasing the nesting depth unnecessarily, the `else` and
following `if` may be combined as `elif`:

```python
if x > 0:
        result = 1
elif x < 0:
        result = -1
else:
        result = 0
```

An `if` statement is permitted only within a function definition.
An `if` statement at top level results in a static error.
"#;

pub(crate) const FOR_DOCS: &str = r#"A `for` loop evaluates its operand, which must be an iterable value.
Then, for each element of the iterable's sequence, the loop assigns
the successive element values to one or more variables and executes a
list of statements, the _loop body_.

```text
ForStmt = 'for' LoopVariables 'in' Expression ':' Suite .
```

Example:

```python
for x in range(10):
   print(10)
```

The assignment of each value to the loop variables follows the same
rules as an ordinary assignment.  In this example, two-element lists
are repeatedly assigned to the pair of variables (a, i):

```python
for a, i in [["a", 1], ["b", 2], ["c", 3]]:
  print(a, i)                          # prints "a 1", "b 2", "c 3"
```

Because Starlark loops always iterate over a finite sequence, they are
guaranteed to terminate, unlike loops in most languages which can
execute an arbitrary and perhaps unbounded number of iterations.

Within the body of a `for` loop, `break` and `continue` statements may
be used to stop the execution of the loop or advance to the next
iteration.

In Starlark, a `for` loop is permitted only within a function definition.
A `for` loop at top level results in a static error.
"#;

pub(crate) const BREAK_DOCS: &str = r#"
```python
break
```

The `break` and `continue` statements terminate the current iteration
of a `for` loop.  Whereas the `continue` statement resumes the loop at
the next iteration, a `break` statement terminates the entire loop.

```text
BreakStmt    = 'break' .
ContinueStmt = 'continue' .
```

Example:

```python
for x in range(10):
    if x%2 == 1:
        continue        # skip odd numbers
    if x > 7:
        break           # stop at 8
    print(x)            # prints "0", "2", "4", "6"
```

Both statements affect only the innermost lexically enclosing loop.
It is a static error to use a `break` or `continue` statement outside a
loop.
"#;

pub(crate) const CONTINUE_DOCS: &str = r#"
```python
continue
```

The `break` and `continue` statements terminate the current iteration
of a `for` loop.  Whereas the `continue` statement resumes the loop at
the next iteration, a `break` statement terminates the entire loop.

```text
BreakStmt    = 'break' .
ContinueStmt = 'continue' .
```

Example:

```python
for x in range(10):
    if x%2 == 1:
        continue        # skip odd numbers
    if x > 7:
        break           # stop at 8
    print(x)            # prints "0", "2", "4", "6"
```

Both statements affect only the innermost lexically enclosing loop.
It is a static error to use a `break` or `continue` statement outside a
loop.
"#;

pub(crate) const LOAD_DOCS: &str = r#"
```python
load
```

The `load` statement loads another Starlark module, extracts one or
more values from it, and binds them to names in the current module.

<!--
The awkwardness of load statements is a consequence of staying a
strict subset of Python syntax, which allows reuse of existing tools
such as editor support. Python import statements are inadequate for
Starlark because they don't allow arbitrary file names for module names.
-->

Syntactically, a load statement looks like a function call `load(...)`.

```text
LoadStmt = 'load' '(' string {',' [identifier '='] string} [','] ')' .
```

A load statement requires at least two "arguments".
The first must be a literal string; it identifies the module to load.
Its interpretation is determined by the application into which the
Starlark interpreter is embedded, and is not specified here.

During execution, the application determines what action to take for a
load statement.
A typical implementation locates and executes a Starlark file,
populating a cache of files executed so far to avoid duplicate work,
to obtain a module, which is a mapping from global names to values.

The remaining arguments are a mixture of literal strings, such as
`"x"`, or named literal strings, such as `y="x"`.

The literal string (`"x"`), which must denote a valid identifier not
starting with `_`, specifies the name to extract from the loaded
module.  In effect, names starting with `_` are not exported.
The name (`y`) specifies the local name;
if no name is given, the local name matches the quoted name.

```python
load("module.sky", "x", "y", "z")       # assigns x, y, and z
load("module.sky", "x", y2="y", "z")    # assigns x, y2, and z
"#;

pub(crate) const PASS_DOCS: &str = r#"
```python
pass
```

A `pass` statement does nothing.  Use a `pass` statement when the
syntax requires a statement but no behavior is required, such as the
body of a function that does nothing.

```text
PassStmt = 'pass' .
```

Example:

```python
def noop():
   pass

def list_to_dict(items):
  # Convert list of tuples to dict
  m = {}
  for k, m[k] in items:
    pass
  return m
```
"#;
