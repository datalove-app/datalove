import Lean
import HVM.Codegen.CodegenM
import HVM.Codegen.Overrides.All
import HVM.Datatypes.Term
import HVM.DSL

-- import Yatima.CodeGen.CodeGen
-- import Yatima.Cli.Utils
-- import Cli.Basic

namespace HVM.Codegen

open HVM Datatypes DSL Term
open Lean.Compiler.LCNF

/--
This is a super dangerous instance, because of how tricky names are;
I'm just gonna turn it on for now, but may cause terrible bugs.
-/
scoped instance (priority := low) : ToTerm Lean.Name where
  toTerm name := .atm (.ident name.toString)

instance : ToTerm Lean.FVarId where
  toTerm fvarId := toTerm fvarId.name

instance : ToTerm LitValue where toTerm
  | .natVal n => toTerm n
  | .strVal s => toTerm s

def preloads : List (Lean.Name × Rule) := [
  -- Lurk.Preloads.throw,
  -- Lurk.Preloads.reverse_aux,
  -- Lurk.Preloads.reverse,
  -- Lurk.Preloads.set,
  -- Lurk.Preloads.set!,
  -- Lurk.Preloads.push,
  -- Lurk.Preloads.append,
  -- Lurk.Preloads.getelem!,
  -- Lurk.Preloads.drop,
  -- Lurk.Preloads.str_mk,
  -- Lurk.Preloads.str_data,
  -- Lurk.Preloads.str_push,
  -- Lurk.Preloads.str_append,
  -- Lurk.Preloads.to_bool,
  -- Lurk.Preloads.lor,
  -- Lurk.Preloads.land,
  -- Lurk.Preloads.lnot,
  -- Lurk.Preloads.lneq
]

def preloadNames : Lean.NameSet := .ofList (preloads.map Prod.fst)

def safeName (name : Lean.Name) : CodegenM Lean.Name :=
  let nameStr := name.toString false
  if preloadNames.contains name || nameStr.contains '|' then do
    match (← get).replaced.find? name with
    | some n => return n
    | none   => replace name
  else return name

private def mkName (name : Lean.Name) : CodegenM Term :=
  toTerm <$> safeName name

@[inline] private def mkFVarId (fvarId : Lean.FVarId) : CodegenM Term :=
  mkName fvarId.name

private def mkArg : Arg → CodegenM Term
  | .erased => return .nil
  | .fvar fvarId => mkFVarId fvarId
    -- hopefully can erase types??
  | .type _ => return .nil

private def mkParam : Param → CodegenM String
  | ⟨fvarId, _, _, _⟩ =>
    dbg_trace s!"-> mkParam {fvarId.name}"
    -- return ← mkFVarId fvarId
    return (← safeName fvarId.name).toString false

private def mkParams (params : Array Param) : CodegenM (Array String) :=
  return ← params.mapM fun p => mkParam p

-- private def mkRule (name : Lean.Name) (args : List Param) (body : Term) : CodegenM Rule := do
--   let params ← mkParams args.toArray
--   dbg_trace s!"-> mkCtr {name} {params}"
--   -- let args := args.map (·.toTerm false)
--   let ctr := Term.ctr name.toString params
--   Rule {
--     name := name,
--     lhs := ctr,
--     rhs := body
--   }

private def mkCtr (f : String) (args : List Term) : Term :=
  dbg_trace s!"-> mkApp: {f} {args}"
  -- args.foldl (init := f) fun acc e => .app acc e
  Term.ctr f args.toArray

/-FIXME:-/
private def mkApp (f : Term) (args : List Term) : Term :=
  dbg_trace s!"-> mkApp: {f} {args}"
  -- args.foldl (init := f) fun acc e => .app acc e
  Term.app f args.toArray

/-FIXME:-/
private def mkLambda (args : List String) (body : Term) : Term :=
  dbg_trace s!"-> mkLambda: {args} {body}"
  args.foldr (init := body) fun s acc => .lam s acc

/-FIXME:-/
private def mkLet (binders : List $ String × Term) (body : Term) : Term :=
  binders.foldr (init := body) fun (s, v) acc => .let s v acc

/-FIXME:-/
private def mkIfElses (ifThens : List (Term × Term)) (finalElse : Term := .nil) : Term :=
  match ifThens with
  | [] => .nil
  | [(cond, body)] =>
    -- .if cond body finalElse
    .nil
  | (cond, body) :: es =>
    -- .if cond body (mkIfElses es finalElse)
    .nil

/-FIXME:-/
def mkCasesCore (discr : Term) (alts : Array Override.Alt) : CodegenM Term := do
  dbg_trace s!"-> mkCasesCore: {discr}"
  let mut defaultElse : Term := .nil
  let mut ifThens : Array (Term × Term) := #[]
  for alt in alts do match alt with
    | .default k => defaultElse := k
    | .alt cidx params k =>
        ifThens := ifThens
      -- if params.isEmpty then
      --   -- todo:
      --   -- ifThens := ifThens.push (⟦(= "_lurk_idx" $cidx)⟧, k)
      -- else
      --   let params : List (String × Term) := params.toList.foldr (init := [])
      --     fun param acc =>
      --       -- todo:
      --       -- (param.toString false, ⟦(car _lurk_args)⟧) :: ("_lurk_args", ⟦(cdr _lurk_args)⟧) :: acc
      --       -- ⟦0⟧ :: acc
      --       acc
      --   let case := mkLet params k
      --   -- todo:
      --   ifThens := ifThens.push (⟦(= _lurk_idx $cidx)⟧, case)
  let cases := mkIfElses ifThens.toList defaultElse
  -- I have to write it like this because Lean is having a hard time elaborating stuff
  -- let lurk_idx : Expr := ⟦(car $discr)⟧
  -- return ⟦(let ((_lurk_idx $lurk_idx)
  --               (_lurk_args (drop 1 $discr)))
  --           $cases)⟧
  return .nil

/--

FIXME:
-/
def mkIndLiteral (ind : Lean.InductiveVal) : CodegenM Term := do
  let (name, params, indices, type) :=
    (ind.name.toString false, ind.numParams, ind.numIndices, ind.type)
  let args ← type.getForallBinderNames.mapM safeName
  let args := args.map (·.toString false)
  -- if args.isEmpty then
  --   -- return ⟦,($name $params $indices)⟧
  -- else
  --   -- return .mkLambda args ⟦,($name $params $indices)⟧
  return .nil

private def addRule (r : Rule) (safe := true) : CodegenM Unit := do
  -- let name := if safe then ← safeName r.name else r.name
  modify fun s => { s with rules := s.rules.push r }

private def addInductiveData (data : InductiveData) : CodegenM Unit := do
  modify fun s => { s with inductives := s.inductives.insert data.name data }

/--
Appends a `Lean.ConstructorVal`
TODO:
-/
private def addConstructor (ctor : Lean.ConstructorVal) : CodegenM Unit := do
  visit ctor.name
  let ctorArgs ← ctor.type.getForallBinderNames.mapM safeName
  let ctorArgNames := ctorArgs.drop ctor.numParams
  -- let ctorArgNames := ⟦(cons $ctor.cidx $(mkConsListWith $ ctorArgNames.map toTerm))⟧
  -- let body := if ctorArgs.isEmpty then
  --   ctorArgNames
  -- else
  --   .mkLambda (ctorArgs.map (·.toString false)) ctorData
  -- addRule Rule { name := ctor.name, lhs :=  rhs := body }

/--
Appends an `Lean.InductiveVal`

Amazingly, we don't actually have to codeGen recursors...
FIXME:
-/
private def addInductive (ind : Lean.InductiveVal) : CodegenM Unit := do
  let name := ind.name
  visit name
  let ctors : List Lean.ConstructorVal ← ind.ctors.mapM fun ctor => do
    match (← read).env.constants.find? ctor with
    | some (.ctorInfo ctor) => return ctor
    | _ => throw s!"malformed environment, {ctor} is not a constructor or doesn't exist"
  -- let ctorData := ctors.foldl (init := .empty)
  --   fun acc ctor => acc.insert ctor.name ctor.cidx
  -- addInductiveData ⟨name, ind.numParams, ind.numIndices, ctorData⟩
  -- appendBinding (name, ← mkIndLiteral ind)
  -- for ctor in ctors do
  --   addConstructor ctor
  return ()

mutual

  /-FIXME:-/
  partial def mkFunDecl : FunDecl → CodegenM (String × Term)
    | ⟨fvarId, _, params, _, value⟩ => do
      dbg_trace s!"---> mkFunDecl: {fvarId.name}"
      let fvarId ← safeName fvarId.name
      let fvarId := fvarId.toString false
      let value ← mkCode value
      let ⟨params⟩ ← mkParams params
      return (fvarId, mkLambda params value)

  /-FIXME:-/
  partial def mkLetDecl : LetDecl → CodegenM (String × Term)
    | ⟨fvarId, _, _, value⟩ => do
      dbg_trace s!"---> mkLetDecl: {fvarId.name}"
      let fvarId ← safeName fvarId.name
      let fvarId := fvarId.toString false
      let value ← mkLetValue value
      return (fvarId, value)

  /-FIXME:-/
  partial def mkLetValue (letv : LetValue) : CodegenM Term :=
    match letv with
    | .erased =>
      dbg_trace s!"---> mkLetValue .erased"
      return .nil
    | .value lit =>
      dbg_trace s!"---> mkLetValue .value {lit.toExpr}"
      return toTerm lit
    | .proj typeName idx struct => do
      dbg_trace s!"---> mkLetValue .proj {typeName} {idx} {struct.name}"
      addName typeName
      -- -- TODO FIXME: use `typeName` to get params and add to `idx`
      -- -- TODO FIXME: support overrides; this is somewhat non-trivial
      -- return ⟦(getelem! $struct.name $(1 + idx))⟧
      return .nil
    | .const declName _ args => do
      dbg_trace s!"---> mkLetValue .const {declName} isEmpty: {args.isEmpty}"
      addName declName
      if args.isEmpty then return toTerm declName
      else return mkApp (toTerm declName) $ (← args.mapM mkArg).data
    | .fvar fvarId args =>
      dbg_trace s!"---> mkLetValue .const {fvarId.name} isEmpty: {args.isEmpty}"
      if args.isEmpty then mkName fvarId.name
      else return mkApp (← mkFVarId fvarId) $ (← args.mapM mkArg).data

  /-FIXME:-/
  partial def mkCases (cases : Cases) : CodegenM Term := do
    let ⟨typeName, _, discr, alts⟩ := cases
    dbg_trace s!"---> mkCases: {typeName} discr.name: {discr.name}"

    addName typeName
    let indData := ← match (← get).inductives.find? typeName with
      | some data => return data
      | none => throw s!"{typeName} is not an inductive"
    let discr ← mkFVarId discr
    let alts ← mkOverrideAlts indData alts
    match (← read).overrides.find? typeName with
    | some (.ind ind) => liftExcept <| ind.mkCases discr alts
    | none            => mkCasesCore discr alts
    | some (.decl _)  => throw s!"found a declaration override for {typeName}"

  partial def mkOverrideAlt (indData : InductiveData) : Alt → CodegenM Override.Alt
    | .default k => .default <$> mkCode k
    | .alt ctor params k => do
      let some cidx := indData.ctors.find? ctor |
        throw s!"{ctor} not a valid constructor for {indData.name}"
      let params ← params.mapM fun p => safeName p.fvarId.name
      return .alt cidx params (← mkCode k)

  partial def mkOverrideAlts (indData : InductiveData) (alts : Array Alt) :
      CodegenM (Array Override.Alt) := do
    alts.mapM $ mkOverrideAlt indData

  /-
  FIXME:
  -/
  partial def mkCode : Code → CodegenM Term
    | .let decl k => do
      dbg_trace s!"---> mkCode .let: {decl.fvarId.name} {decl.binderName}"
      let (name, decl) ← mkLetDecl decl
      let k ← mkCode k
      return .let name decl k
    | .fun decl k | .jp decl k => do -- `.fun` and `.jp` are the same case to Lurk
      dbg_trace s!"---> mkCode .fun/.jp: {decl.fvarId.name} {decl.binderName}"
      let (name, decl) ← mkFunDecl decl
      let k ← mkCode k
      return .let name decl k
    | .jmp fvarId args => do
      dbg_trace s!"---> mkCode .jmp: {fvarId.name}"
      let fvarId ← mkFVarId fvarId
      let args ← args.mapM mkArg
      return mkApp fvarId args.data
    | .cases cases =>
      dbg_trace s!"---> mkCode .cases: {cases.typeName}"
      mkCases cases
    | .return fvarId =>
      dbg_trace s!"---> mkCode .return: {fvarId.name}"
      mkFVarId fvarId
    | .unreach u =>
      dbg_trace s!"---> mkCode .unreach {u}"
      return .nil

  partial def mkDecl! : Code → CodegenM Term
    -- | .let decl k => do
    --   dbg_trace s!"---> mkCode .let: {decl.fvarId.name} {decl.binderName}"
    --   let (name, decl) ← mkLetDecl decl
    --   let k ← mkCode k
    --   return .ctr name.toUpper decl k
    | code => mkCode code

  /-FIXME:-/
  partial def addDecl (decl : Decl) : CodegenM Unit := do
    let ⟨name, _, _, params, value, _, _, _⟩ := decl
    visit name
    -- dbg_trace s!"-> addDecl mkCode call: {name} {params.map fun p => p.binderName.toString}"
    -- let ⟨params⟩ := params.map fun p => p.fvarId.name.toString false
    let ⟨params⟩ ← params.mapM fun p => mkFVarId p.fvarId
    let ctr := mkCtr name.toString params
    let value ← mkDecl! value

    dbg_trace s!"-> addDecl mkCode: {ctr} value: {params}"

    -- let body := if !params.isEmpty then
    --   dbg_trace s!"-> addDecl no params: {name}"
    --   dbg_trace s!"-> addDecl w/ params: {name} {params.map ToString.toString}"
    --   mkCtr name params
    -- dbg_trace s!"-> addDecl body: {name} {body}"
    addRule (Rule.mk name ctr value) false

  /-FIXME:-/
  partial def addOverride (name : Lean.Name) : CodegenM Bool := do
    match (← read).overrides.find? name with
    | some (.decl ⟨name, decl⟩) =>
      visit name
      -- todo: appendPrereqs decl
      -- todo: addRules (name, decl)
      dbg_trace s!"-> addOverride found override.decl: {name} {decl}"
      return true
    | some (.ind ⟨indData, ⟨name, decl⟩, ctors, _⟩) =>
      visit name
      addInductiveData indData
      -- todo: appendPrereqs decl
      -- todo: addRule (name, decl)
      for ⟨name, ctor⟩ in ctors do
        visit name
        -- todo: appendPrereqs ctor
        -- todo: addRule (name, ctor)
      dbg_trace s!"-> addOverride found override.ind: {name} {decl}"
      return true
    | none =>
      dbg_trace s!"-> addOverride no override: {name}"
      return false
  -- where
  --   appendPrereqs (x : Term) : CodegenM Unit :=
  --     (x.getFreeVars).toList.forM fun n => do
  --       let n := n.toNameSafe
  --       if !(← isVisited n) then addName n

  /--
  Entrypoint for code generation. Adds overrides and either inductives or a declaration.

  FIXME:
  -/
  partial def addName (name : Lean.Name) : CodegenM Unit := do
    dbg_trace s!"-> addName: {name}"
    if ← isVisited name then return
    match ← getCtorOrIndInfo? name with
    | some inds =>
      dbg_trace s!"===> addName found ctorOrIndInfo: {name}"
      for ind in inds do
        if ← addOverride ind then continue
        let ind ← getInductive! ind
        dbg_trace s!"===> addName found inductiveVal {ind.name} in {name}"
        addInductive ind
    | none =>
      dbg_trace s!"===> addName no ctorOrIndInfo: {name}"
      if ← addOverride name then return
      let decl := ← getDecl! name
      dbg_trace s!"===> addName found decl: {decl.name}"
      addDecl decl

end

-- /--
-- Transforms a list of named expressions that were mutually defined into a
-- "switch" function `S` and a set of projections (named after the original names)
-- that call `S` with their respective indices.

-- For example, suppose we have two binders `(a (+ a 1))` and `(b (+ b 2))`.
-- Calling `mkMutualBlock` on them will generate the binders:

-- 1. (mut_a_b (LAMBDA (key)
--     (IF (= key 0)
--       (+ (mut_a_b 0) 1)
--       (IF (= key 1)
--         (+ (mut_a_b 1) 2)
--         NIL))))
-- 2. (a (mut_a_b 0))
-- 3. (b (mut_a_b 1))

-- Important: the resulting binders must be in a `letrec` block.
-- -/
-- def mkMutualBlock
--   (binders : List $ String × Expr)
--   (init := "mut")
--   (merge : String → String → String := fun acc n => s!"{acc}_{n}")
--   (key := "key") :
--     List $ String × Expr :=
--   match binders with
--   | x@([ ])
--   | x@([_]) => x
--   | _ =>
--     let names := binders.map Prod.fst
--     let mutualName := names.foldl merge init
--     let projs := names.enum.map fun (i, n) =>
--       (n, .app (.sym mutualName) (Expr.toExpr i))
--     let map := projs.foldl (init := default) fun acc (n, e) => acc.insert n e
--     let keySym := Expr.sym key
--     let ifThens := binders.enum.map
--       fun (i, (_, e)) => (.op₂ .numEq keySym (toExpr i), e.replaceFreeVars map)
--     let mutualBlock := mkIfElses ifThens
--     (mutualName, .lambda key mutualBlock) :: projs

-- /--
-- Given a list of binders which are naively mutually recursive,
-- collect all the strongly connected components and then make them into mutual blocks.
-- -/
-- def mutualize (binders : List $ String × Expr) : List $ String × Expr :=
--   let names := binders.map Prod.fst
--   let binders := RBMap.ofList binders compare
--   let blocks := Lean.SCC.scc names fun name =>
--     binders.find! name |>.getFreeVars default default |>.toList
--   List.join <| blocks.map fun block =>
--     let block := block.map fun name => (name, binders.find! name)
--     mkMutualBlock block

/--
Main code generation function
-/
-- open HVM.Overrides
def codeGenM (decl : Lean.Name) : CodegenM Unit :=
  -- TODO: our overrides
  let overrides := .ofList $ Overrides.All.module.map fun o => (o.name, o)
  withOverrides overrides do
    preloads.forM fun (name, preload) => do
      visit name
      -- appendBinding (name, preload) false
    addName decl

def codeGen (env : Lean.Environment) (decl : Lean.Name) : Except String Rulebook :=
  match CodegenM.run ⟨env, .empty⟩ default (codeGenM decl) with
  | .error e _ => .error e
  | .ok _ s => do
    -- let bindings := s.appendedBindings.map fun (name, body) =>
    --   Rule { name := name, lhs := body, rhs := body }

    return s.rules.toList



-- open System Yatima.CodeGen in
-- def codeGenRun (p : Cli.Parsed) : IO UInt32 := do
--   -- Parse Lean file and target declaration
--   let some source := p.positionalArg? "source" |>.map (·.value)
--     | IO.eprintln "No source was provided"; return 1
--   let some decl := p.flag? "decl" |>.map (·.value.toNameSafe)
--     | IO.eprintln "No declaration provided"; return 1

--   -- Compute Lurk expression
--   Lean.setLibsPaths
--   let path := ⟨source⟩
--   let expr ← match codeGen (← Lean.runFrontend (← IO.FS.readFile path) path) decl with
--   | .error msg => IO.eprintln msg; return 1
--   | .ok expr => pure $ if p.hasFlag "anon" then expr.anon else expr

--   -- Write Lurk file
--   let output := match p.flag? "lurk" |>.map (·.value) with
--     | some output => ⟨output⟩
--     | none => ⟨s!"{decl}.lurk"⟩

--   IO.println s!"Writing output to {output}"
--   IO.FS.writeFile output (expr.toString true)

--   -- Run if requested
--   if p.hasFlag "run" then
--     match expr.evaluate with
--     | .ok (val, iterations) =>
--       IO.println s!"Iterations: {iterations}"
--       IO.println val
--     | .error (err, frames) =>
--       IO.eprintln err
--       let nFrames := (p.flag? "frames").map (·.as! Nat) |>.getD 5
--       let framesFilePath := output.withExtension "frames"
--       IO.FS.writeFile framesFilePath (frames.pprint nFrames)
--       IO.eprintln s!"Dumped {nFrames} frames to {framesFilePath}"
--       return 1
--   else if p.hasFlag "lurkrs" then
--     match ← Lean.runCmd "lurkrs" #[output.toString] with
--     | .ok res => IO.print res; return 0
--     | .error err => IO.eprint err; return 1

--   return 0


-- open Lurk Scalar Expr.DSL DSL in
-- def proveRun (p : Cli.Parsed) : IO UInt32 := do
--   let some (stt : LDONHashState) ← loadData LDONHASHCACHE false | return 1

--   -- Get environment file name
--   let some decl := p.positionalArg? "decl" |>.map (·.value.toNameSafe)
--     | IO.eprintln "No declaration was provided"; return 1

--   -- Load environment
--   let some envFileName := p.flag? "env" |>.map (·.value)
--     | IO.eprintln "Environment file not provided"; return 1
--   let some (env : Yatima.IR.Env) ← loadData envFileName false | return 1

--   let some declComm := env.consts.find? decl
--     | IO.eprintln s!"{decl} not found in the environment"; return 1

--   let storeFileName : System.FilePath :=
--     p.flag? "store" |>.map (·.value) |>.getD ⟨s!"{decl}.ldstore"⟩

--   let output := match p.flag? "lurk" |>.map (·.value) with
--     | some output => ⟨output⟩
--     | none => s!"{decl}.lurk"

--   let mut expr := default
--   let mut store := default

--   if p.hasFlag "raw-tc" then
--     let tcExpr ← match ← genTypechecker with
--       | .error msg => IO.eprintln msg; return 1
--       | .ok expr' => pure expr'

--     -- simply apply the typechecker to the constant hash
--     expr := mkRawTypecheckingExpr tcExpr declComm

--     -- setting up the store
--     store ← match stt.extractComms env.hashes with
--       | .error err => IO.eprintln err; return 1
--       | .ok store' => pure store'
--   else
--     let some (tcComm : F) ← loadData TCHASH false | return 1

--     -- call `eval` on the typechecker committed as LDON
--     expr := mkCommTypecheckingExpr tcComm declComm

--     -- setting up the store
--     store ← match stt.extractComms (env.hashes.push tcComm) with
--       | .error err => IO.eprintln err; return 1
--       | .ok store' => pure store'

--   -- Write the store
--   dumpData store storeFileName

--   -- Write Lurk file
--   IO.FS.writeFile output s!"{expr.toFormat true}"

--   -- Run if requested
--   if p.hasFlag "run" then
--     match expr.evaluate store with
--     | .ok (v, n) =>
--       IO.println s!"[{n} evaluations] => {v}"
--     | .error (err, frames) =>
--       IO.eprintln err
--       let nFrames := (p.flag? "frames").map (·.as! Nat) |>.getD 5
--       let framesFilePath := output.withExtension "frames"
--       IO.FS.writeFile framesFilePath (frames.pprint nFrames)
--       IO.eprintln s!"Dumped {nFrames} frames to {framesFilePath}"
--       return 1
--   else if p.hasFlag "lurkrs" then
--     match ← Lean.runCmd "lurkrs" #[output.toString] with
--     | .ok res => IO.print res; return 0
--     | .error err => IO.eprint err; return 1

--   return 0

end HVM.Codegen
