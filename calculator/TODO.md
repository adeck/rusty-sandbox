
Still need to get a firm handle on:

- lifetimes
  - generally, but
  - especially as they apply to heap-allocation
- mutability
  - when to pass mutable v. immutable references

Need to fix the way I use:

- match statements
  - Need to use `use` declarations and / or lambdas so I don't end up
    having insanely stupid-looking match statements that look like:
    match x {
      A(B(c)) => X::Y::Z(J::K(L)),
      A(B(d) => X::Y::Z(J::K(M)),
      A(_) => X::Y::Z(J::N),
      _ => P::Q
    }
    where instead I could use something more like:
    {
      use J; use X::Y::Z as Z;
      match x {
        A(B(c)) => Z(K(L)),
        A(B(d) => Z(K(M)),
        A(_) => Z(N),
        _ => P::Q
      }
    }


