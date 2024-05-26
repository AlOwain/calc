# calc

# Note of Archival

This will be temporarily archived, I might still continue developing this in the future.

Learning rust by taking inspiration from [numi](https://github.com/nikolaeu/numi).

### Short Term Goals

- [ ] Create an interactive runtime
- [ ] Support reading from files

### Long Term Goals

- [ ] Extensibility: This program must support function plugins, you add a plugin with a definition along those lines:

```rust
// Of course given that rust is a compiled language extensibility would be limited,
// but nevertheless it should happen, we could use lua or something.
register(Operator::Add, '+', handle_add);
// ...
fn handle_add(lhs: &Operand, rhs: &Operand) -> Operand {
    lhs + rhs
}
```

- [ ] Lenient: This program should be as lenient as possible, it should try to do whatever you throw at it.
- [ ] Script-ish: Every line should have a result, and it should be printed out.
- [ ] Symbol-based: Every operator is a symbol, `or` should be `∨` `and` should be `∧`, and so on..
- The user could choose to type `or(lhs, rhs)` or `and(lhs, rhs)` but it should map (in-place) whenever it is run into `∨` and `∧`.
- A config option should be there to set mapping on or off (on by default).
- The mapping only happens after the program runs successfully.
- A pre-mapped version of every run file must be cached in case the user wants to return it to its original state.
- [ ] Floating-point: As much as possible the program must map to actual results, 0.30000000000000004 is unnaceptable.
- [ ] Overflow: As much as possible the program must avoid overflow in calculation.
