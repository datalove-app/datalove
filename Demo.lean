-- import Init.Data.Nat

-- def zero : Nat := Nat.zero
-- def succ (n : Nat) := n.succ

-- def add (c : Nat) (d : Nat) : Nat := Nat.add c d
def add (c : Nat) (d : Nat) : Nat :=
  let a := Nat.mul c 1
  let b := Nat.mul d 1
  Nat.add a b
