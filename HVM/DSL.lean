import Lean
import HVM.AST

namespace HVM.DSL

open HVM.AST
open Lean Elab Meta

/-
term ::=
  | λvar. term               # a lambda
  | (term term)              # an application
  | (ctr term term ...)      # a constructor
  | num                      # a machine int
  | (op2 term term)          # an operator
  | let var = term; term     # a local definition

rule ::=
  | term = term
-/

-- declare_syntax_cat    sym
-- scoped syntax "Nil" : sym
-- scoped syntax ident : sym

-- def elabSym : TSyntax `sym → TermElabM Lean.Expr
--   | `(sym| if) => mkAppM ``Sym.if #[]
--   | _ => throwUnsupportedSyntax

declare_syntax_cat        atom_
scoped syntax "Nil"     : atom_
scoped syntax "U60" num : atom_
scoped syntax "F60" num : atom_
scoped syntax str       : atom_
scoped syntax ident     : atom_

def elabAtom : TSyntax `atom_ → TermElabM Lean.Expr
  | `(atom_| Nil) => mkAppM ``Atom.nil #[]
  | `(atom_| U60 $n:num) => mkAppM ``Atom.u60 #[mkNatLit n.getNat]
  -- | `(atom_| F60 $n:num) => mkAppM ``Atom.f60 #[mkFloatLit n.getNat]
  | _ => throwUnsupportedSyntax

declare_syntax_cat    op₁
scoped syntax "!"   : op₁

def elabOp₁ : TSyntax `op₁ → TermElabM Lean.Expr
  | `(op₁| !) => return mkConst ``Op₁.not
  | _ => throwUnsupportedSyntax

declare_syntax_cat        op₂
scoped syntax "Cons"    : op₂
scoped syntax "+"       : op₂
scoped syntax "-"       : op₂
scoped syntax "*"       : op₂
scoped syntax "/"       : op₂
scoped syntax "%"       : op₂
scoped syntax "&"       : op₂
scoped syntax "|"       : op₂
scoped syntax "^"       : op₂
scoped syntax "≪"       : op₂
scoped syntax "≫"       : op₂
scoped syntax "<"       : op₂
scoped syntax ">"       : op₂
scoped syntax "<="      : op₂
scoped syntax ">="      : op₂
scoped syntax "="       : op₂
scoped syntax "≠"       : op₂

def elabOp₂ : TSyntax `op₂ → TermElabM Lean.Expr
  | `(op₂| Cons)  => return mkConst ``Op₂.cons
  | `(op₂| +)     => return mkConst ``Op₂.add
  | `(op₂| -)     => return mkConst ``Op₂.sub
  | `(op₂| *)     => return mkConst ``Op₂.mul
  | `(op₂| /)     => return mkConst ``Op₂.div
  | `(op₂| %)     => return mkConst ``Op₂.mod
  | `(op₂| &)     => return mkConst ``Op₂.and
  | `(op₂| |)     => return mkConst ``Op₂.or
  | `(op₂| ^)     => return mkConst ``Op₂.xor
  | `(op₂| ≪)     => return mkConst ``Op₂.shl
  | `(op₂| ≫)     => return mkConst ``Op₂.shr
  | `(op₂| <)     => return mkConst ``Op₂.lt
  | `(op₂| >)     => return mkConst ``Op₂.gt
  | `(op₂| <=)    => return mkConst ``Op₂.le
  | `(op₂| >=)    => return mkConst ``Op₂.ge
  | `(op₂| =)     => return mkConst ``Op₂.eq
  | `(op₂| ≠)     => return mkConst ``Op₂.neq
  -- | `(atom_| f60 $n:num) => mkAppM ``Atom.num #[mkFloatLit n.getNat]
  | _ => throwUnsupportedSyntax

-- declare_syntax_cat          op₃
-- scoped syntax "U60.if"    : op₃
-- scoped syntax "U60.swap"  : op₃

-- def elabOp₃ : TSyntax `op₃ → TermElabM Lean.Expr
--   | `(op₃| U60.if)    => mkAppM ``Atom.ident #[mkStrLit "U60.if"]
--   | `(op₃| U60.swap)  => mkAppM ``Atom.ident #[mkStrLit "U60.swap"]
--   | _ => throwUnsupportedSyntax

declare_syntax_cat                                      term_
-- lambda
scoped syntax atom_                                   : term_
scoped syntax num                                     : term_
scoped syntax str                                     : term_
scoped syntax ident                                   : term_
scoped syntax "let" ident "=" term_ ";" term_         : term_
scoped syntax "λ" ident term_                         : term_
-- ops
scoped syntax "(" op₁ term_ ")"                       : term_
scoped syntax "(" op₂ term_ term_ ")"                 : term_
scoped syntax "(" "if" term_ term_ term_ ")"          : term_
-- ctr
scoped syntax (priority := low) "(" ident term_+ ")"  : term_
-- app
-- scoped syntax (priority := low) "(" term_ "[" term_* "]" ")"  : term_

partial def elabTerm : TSyntax `term_ → TermElabM Lean.Expr
  -- atom
  | `(term_| $a:atom_) => do
    mkAppM ``Term.atm #[← elabAtom a]
  | `(term_| $n:num) => do
    mkAppM ``Term.atm #[← mkAppM ``Atom.u60 #[mkNatLit n.getNat]]
  | `(term_| $s:str) => do
    mkAppM ``Term.atm #[← mkAppM ``Atom.str #[mkStrLit s.getString]]
  | `(term_| $s:ident) => do
    mkAppM ``Term.atm #[← mkAppM ``Atom.ident #[mkStrLit s.getId.toString]]

  -- ops
  | `(term_| ($o:op₁ $e)) => do
    mkAppM ``Term.op₁ #[← elabOp₁ o, ← elabTerm e]
  | `(term_| ($o:op₂ $e₁ $e₂)) => do
    mkAppM ``Term.op₂ #[← elabOp₂ o, ← elabTerm e₁, ← elabTerm e₂]
  | `(term_| (if $e₁ $e₂ $e₃)) => do
    let ctr ← mkAppM ``Term.atm #[← mkAppM ``Atom.ident #[mkStrLit "U60.if"]]
    let args ← mkAppM ``Array.mk #[← elabTerm e₁, ← elabTerm e₂, ← elabTerm e₃]
    mkAppM ``Term.ctr #[ctr, args]
  -- | `(term_| λ $x:ident . $b) => do
    -- let x ← elabBinder x
    -- let b ← elabTerm b
    -- Lean.mkLambdaFVars #[x] b

  -- ctr
  | `(term_| ($n:ident $a:term_*)) => do
    -- dbg_trace s!"ctr terms: {a}"
    let args ← mkArrayLit (mkConst `HVM.Datatypes.Term) (← a.mapM elabTerm).toList
    mkAppM ``Term.ctr #[mkStrLit n.getId.toString, args]

  -- -- app
  -- | `(term_| ($n:term_ [ $a:term_* ])) => do
  --   let args ← mkAppM ``Array.mk (← a.mapM elabTerm)
  --   mkAppM ``Term.app #[← elabTerm n, args]

  | _ => throwUnsupportedSyntax

scoped elab "⟦" e:term_ "⟧" : term =>
  elabTerm e

end HVM.DSL
