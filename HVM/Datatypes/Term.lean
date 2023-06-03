import Lean
-- import HVM.Datatypes.Field


namespace HVM.Datatypes

open Std
-- open Init Data


inductive Atom
  | nil
  | u60 : Nat → Atom
  | f60 : Float → Atom
  | str : String → Atom
  | ident : String → Atom
  deriving Inhabited, Repr, BEq

def Atom.toFormat : Atom → Format
  | .nil    => "Nil"
  | .u60 n  => toString n
  | .f60 n  => toString n
  | .str s  => s!"{s}"
  | .ident s => s

def Atom.toString := ToString.toString ∘ Atom.toFormat

instance : Std.ToFormat Atom := ⟨Atom.toFormat⟩

/-
- Ops
-/
inductive Op₁
  | not
  | log
  | print
  deriving Repr, BEq

def Op₁.toFormat : Op₁ → Format
  | not     => "!"
  | .log    => "HVM.log"
  | .print  => "HVM.print"

def Op₁.toString := ToString.toString ∘ Op₁.toFormat

instance : Std.ToFormat Op₁ := ⟨Op₁.toFormat⟩

inductive Op₂
  | cons
  | add
  | sub
  | mul
  | div
  | mod
  | and
  | or
  | xor
  | shl
  | shr
  | lt
  | gt
  | le
  | ge
  | eq
  | neq
  deriving Repr, BEq

def Op₂.toFormat : Op₂ → Format
  | .cons    => "Cons"
  | .add     => "+"
  | .sub     => "-"
  | .mul     => "*"
  | .div     => "/"
  | .mod     => "%"
  | .and     => "&"
  | .or      => "|"
  | .xor     => "^"
  | .shl     => "≪"
  | .shr     => "≫"
  | .lt      => "<"
  | .gt      => ">"
  | .le      => "<="
  | .ge      => ">="
  | .eq      => "=="
  | .neq     => "!="

def Op₂.toString := ToString.toString ∘ Op₂.toFormat

instance : Std.ToFormat Op₂ := ⟨Op₂.toFormat⟩

/--
- Term

// IC.Terms are Interaction Calculus expressions, which include:
// - variables
// - 2 contructors: lambda and superposition
// - 2 eliminators: application and duplication
// - u60 numbers: included for convenience, but not necessary
type IC.Term {
  // Variables
  var (name: U60)

  // Functions
  lam (x: U60) (body: IC.Term)
  app (fun: IC.Term) (arg: IC.Term)

  // Cloning
  sup (lab: U60) (fst: IC.Term) (snd: IC.Term)
  dup (lab: U60) (a: U60) (b: U60) (expr: IC.Term) (cont: IC.Term)

  // Numbers
  u60 (val: U60)
  nop (op: U60) (n: IC.Term) (m: IC.Term)
}

type Apps.HVM.Term {
  var (name: Apps.HVM.Name)
  dup (nam0: Apps.HVM.Name) (nam1: Apps.HVM.Name) (expr: Apps.HVM.Term) (body: Apps.HVM.Term)
  lam (name: Apps.HVM.Name) (body: Apps.HVM.Term)
  app (func: Apps.HVM.Term) (argm: Apps.HVM.Term)
  ctr (name: Apps.HVM.Name) (args: Data.List Apps.HVM.Term)
  fun (name: Apps.HVM.Name) (args: Data.List Apps.HVM.Term)
  num (numb: Data.U60)
  op2 (oper: Apps.HVM.Oper) (val0: Apps.HVM.Term) (val1: Apps.HVM.Term)
}
-/
inductive Term
  -- nil, lit, var
  | atm : Atom -> Term

  -- -- var
  -- | var : String → Term

  -- λvar term (lambda constructor)
  | lam (x : String) (body : Term) : Term

  -- (term ...) (lambda application eliminator)
  | app (f : Term) (args : Array Term) : Term

  -- (ctr ...) (superposition constructor)
  -- | «fun» : String -> List Term → Term -- ? looks same as ctr
  | ctr (x : String) (args : Array Term) : Term

  -- -- (dup var var term term) (duplication eliminator)
  -- -- .e.g (Foo a) = (+ a a)
  -- | dup : String → String → String → Term → Term → Term

  -- (op1 term)
  | op₁ : Op₁ → Term → Term
  -- (op2 term tern)
  | op₂ : Op₂ → Term → Term → Term

  -- let var = term; term
  | «let» (name : String) (expr : Term) (cont : Term) : Term -- ?
  deriving Inhabited, BEq, Repr

namespace Term

@[match_pattern] def nil : Term := .atm .nil
-- @[match_pattern] def ident (s : String) : Term := .atm (.ident s)

/-! Converts a type to an `HVM.Term` !-/
class ToTerm (α: Type _) where
  toTerm : α → Term

export ToTerm (toTerm)

instance : ToTerm Atom where
  toTerm := .atm

-- instance : ToExpr F where
--   toExpr := .atom ∘ .num

-- TODO: this is bad precision
instance : ToTerm Float where
  toTerm := .atm ∘ .f60

-- TODO: this is bad precision
instance : ToTerm Nat where
  toTerm := .atm ∘ .u60

instance : ToTerm String where
  toTerm := .atm ∘ .str

-- instance : ToTerm Char where
--   toTerm := .atm ∘ .char

instance : ToTerm Term := ⟨id⟩

open Lean (HashSet RBMap)
partial def getFreeVars (bVars acc : HashSet String := default) :
    Term → HashSet String
  -- | .atm (.ident n) => if bVars.contains n then acc else acc.insert n
  | .atm _          => acc
  -- | .sym s => if bVars.contains s then acc else acc.insert s
  | .lam s b        => b.getFreeVars (bVars.insert s) acc
  | .op₁ _ e        => e.getFreeVars bVars acc
  | .op₂ _ e₁ e₂    => e₂.getFreeVars bVars (e₁.getFreeVars bVars acc)
  | .app f args     => args.foldl (fun acc e => e.getFreeVars bVars acc) (f.getFreeVars bVars acc)
  | .ctr _ args     => args.foldl (fun acc e => e.getFreeVars bVars acc) acc
  -- | .if a b c => c.getFreeVars bVars (b.getFreeVars bVars (a.getFreeVars bVars acc))
  | .let    s v b   => b.getFreeVars (bVars.insert s) (v.getFreeVars bVars acc)

/--
Telescopes `(lambda (x₁ x₂ ⋯) body)` into `(#[x₁, x₂, ⋯], body)`
-/
def telescopeLam (acc : Array String := #[]) : Term → (Array String) × Term
  | .lam x body => body.telescopeLam (acc.push x)
  | x => (acc, x)

/--
Telescopes `(let ((n₁ e₁) (n₂ e₂) ⋯) body)` into
`(#[(n₁, e₁), (n₂, e₂), ⋯], body)`
-/
def telescopeLet (acc : Array $ String × Term := #[]) :
    Term → (Array $ String × Term) × Term
  | .let name expr body => body.telescopeLet (acc.push (name, expr))
  | x => (acc, x)

/--
Telescopes `(f a₁ a₂ ⋯)` into `[f, a₁, a₂, ⋯]`
-/
def telescopeApp (acc : List Term) : Term → List Term
  -- | .app f arg => f.telescopeApp (arg :: acc)
  -- | .ctr f arg => f.telescopeApp (arg :: acc)
  | x => x :: acc


-- TODO: this actually outputs the lang as text
open Format
partial def toFormat (esc := false) (e : Term) : Format :=
  have : ToFormat Term := ⟨toFormat⟩
  match e with
  | .atm a => format a
  -- | .var x => formatSym x
  | .lam x body =>
    let (as, b) := body.telescopeLam #[x]
    let as := as.data.map formatSym
    -- paren $ "LAMBDA " ++ nest 2 (paren (joinSep as " ")) ++ indentD (body.toFormat esc)
    -- FIXME:
    "λ" ++ x ++ " " ++ b.toFormat esc
  | .app f args =>
    let as := f.telescopeApp args.toList |>.map $ toFormat esc
    paren (joinSep as " ")
  | .ctr f args =>
    let f := Term.atm (Atom.ident f)
    let as := f.telescopeApp args.toList |>.map $ toFormat esc
    paren (joinSep as " ")
  | .op₁ op e =>
    paren $ format op ++ " " ++ e.toFormat esc
  | .op₂ op e₁ e₂ =>
    paren $ format op ++ " " ++ e₁.toFormat esc ++ " " ++ e₂.toFormat esc
  | .let name body cont =>
    -- let (bs, body) := body.telescopeLet #[(name, expr)]
    -- let bs := bs.data.map fun (n, e) => paren $ formatSym n ++ indentD (e.toFormat esc)
    -- paren $ "LET " ++ nest 4 (paren $ joinSep bs line) ++ indentD (body.toFormat esc)
    "let " ++ name ++ " = " ++ body.toFormat esc ++ ";" ++ line ++ cont.toFormat esc
  -- | .letrec s v b =>
  --   let (bs, b) := b.telescopeLetrec #[(s, v)]
  --   let bs := bs.data.map fun (n, e) => paren $ formatSym n ++ indentD (e.toFormat esc)
  --   paren $ "LETREC " ++ nest 7 (paren $ joinSep bs line) ++ indentD (b.toFormat esc)
  -- | .quote ldon => paren $ "QUOTE" ++ line ++ ldon.toFormat esc
  -- | .eval e env? =>
  --   let env? := if env? == .nil then .nil else line ++ env?.toFormat esc
  --   paren $ "EVAL" ++ line ++ e.toFormat esc ++ env?
where
  formatSym s := if esc then s!"{s}" else s

def toString (esc := false) : Term → String :=
  ToString.toString ∘ toFormat esc

instance : ToFormat Term := ⟨toFormat⟩
instance : ToString Term := ⟨toString⟩

end Term




structure Rule where
  name : Lean.Name
  lhs : Term
  rhs : Term
  deriving Inhabited, BEq, Repr

namespace Rule

open Format
def toFormat (esc := false) (e : Rule) : Format :=
  let rhs := match e.rhs with
  | .lam .. => (e.rhs.toFormat esc)
  | _       => indentD (e.rhs.toFormat esc)
  e.lhs.toFormat esc ++ " = " ++ rhs
def toString (esc := false) : Rule → String :=
  ToString.toString ∘ toFormat esc

instance : ToString Rule := ⟨toString⟩
  -- | ⟨lhs, rhs⟩ => lhs.toString ++ " = " ++ rhs.toString

end Rule


abbrev Rulebook := List Rule

namespace Rulebook
open Format
def toFormat (esc := false) (e : Rulebook) : Format :=
  e.foldl (fun acc r => acc ++ line ++ r.toFormat esc) Format.nil
def toString (esc := false) : Rulebook → String :=
  ToString.toString ∘ toFormat esc

-- instance : ToFormat Rulebook := ⟨toFormat⟩
-- instance : ToString Rulebook := ⟨toString⟩

class ToRulebook (α: Type _) where
  toRulebook : α → Rulebook


end Rulebook

-- import Yatima.Datatypes.Univ
-- import YatimaStdLib.Ord
-- import Lurk.Field

-- namespace Yatima.IR

-- instance (priority := high) : Hashable Literal where hash
--   | .natVal x => hash (0, x)
--   | .strVal x => hash (1, x)

-- inductive Expr
--   /-- Variables are also used to represent recursive calls. When referencing
--     constants, the second argument keeps track of the universe levels -/
--   | var   : Nat → List Univ → Expr
--   | sort  : Univ → Expr
--   | const : Lurk.F → List Univ → Expr
--   | app   : Expr → Expr → Expr
--   | lam   : Expr → Expr → Expr
--   | pi    : Expr → Expr → Expr
--   | letE  : Expr → Expr → Expr → Expr
--   | lit   : Literal → Expr
--   | proj  : Nat → Expr → Expr
--   deriving Inhabited, Ord, BEq, Hashable, Repr

-- end Yatima.IR

end HVM.Datatypes
