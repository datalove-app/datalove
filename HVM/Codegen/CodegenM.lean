-- import Lean
-- import Lean.Compiler.LCNF
import HVM.Datatypes.Term
import HVM.Codegen.Override

namespace HVM.Codegen

open HVM.Datatypes
open Lean.Compiler.LCNF

structure CodegenEnv where
  env : Lean.Environment
  overrides : Lean.NameMap Override

structure CodegenState where
  /-- -/
  rules : Array Rule
  appendedBindings : Array (Lean.Name × Term) -- ? Rule?
  /-- Contains the names of constants that have already been processed -/
  visited  : Lean.NameSet
  inductives : Lean.NameMap InductiveData
  ngen     : Lean.NameGenerator
  replaced : Lean.NameMap Lean.Name
  deriving Inhabited

abbrev CodegenM := ReaderT CodegenEnv $ EStateM String CodegenState

def withOverrides (overrides : Lean.NameMap Override) : CodegenM α → CodegenM α :=
  withReader fun env => { env with overrides := overrides }

instance : Lean.MonadNameGenerator CodegenM where
  getNGen := return (← get).ngen
  setNGen ngen := modify fun s => { s with ngen := ngen }

@[inline] def isVisited (n : Lean.Name) : CodegenM Bool :=
  return (← get).visited.contains n

/-- Set `name` as a visited node -/
def visit (name : Lean.Name) : CodegenM Unit :=
  -- dbg_trace s!">> visit {name}"
  modify fun s => { s with visited := s.visited.insert name }

/-- Create a fresh variable to replace `name` and update `replaced` -/
def replace (name : Lean.Name) : CodegenM Lean.Name := do
  let mut name' ← Lean.mkFreshId
  let env ← read
  while env.env.contains name' || env.overrides.contains name' do
    -- making sure we don't hit an existing name
    name' ← Lean.mkFreshId
  modifyGet fun stt => (name', { stt with
    replaced := stt.replaced.insert name name' })

def getDecl! (name : Lean.Name) : CodegenM Decl := do
  if let some decl := getDeclCore? (← read).env monoExt name then
    dbg_trace s!"=> getDecl! via monoExt {name}"
    return decl
  else if let some decl := getDeclCore? (← read).env baseExt name then
    dbg_trace s!"=> getDecl! via baseExt {name}"
    return decl
  else
    throw s!"environment does not contain {name}"

def getConstructor! (name : Lean.Name) : CodegenM Lean.ConstructorVal := do
  match (← read).env.constants.find? name with
  | some (.ctorInfo name) => return name
  | _ => throw s!"{name} is not a constructor"

def getInductive! (name : Lean.Name) : CodegenM Lean.InductiveVal := do
  match (← read).env.constants.find? name with
  | some (.inductInfo ind) => return ind
  | _ => throw s!"{name} is not an inductive"

def getCtorOrIndInfo? (name : Lean.Name) : CodegenM $ Option (List Lean.Name) := do
  match (← read).env.constants.find? name with
  | some (.inductInfo ind) =>
    dbg_trace s!"=> getCtorOrIndInfo? inductInfo: {ind.name}"
    return some ind.all
  | some (.ctorInfo ctor) =>
    dbg_trace s!"=> getCtorOrIndInfo? ctorInfo: {ctor.induct}"
    let ind ← getInductive! ctor.induct
    return some ind.all
  | _ =>
    dbg_trace s!"=> getCtorOrIndInfo? none"
    return none

def CodegenM.run (env : CodegenEnv) (s : CodegenState) (m : CodegenM α) :
    EStateM.Result String CodegenState α :=
  m env |>.run s

end HVM.Codegen
