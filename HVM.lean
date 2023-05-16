import Lean

import HVM.Codegen.Codegen

-- import Yatima.ContAddr.ContAddr

def toNameSafe (name : String) : Lean.Name :=
  if name.length >= 2 && name.front == '«' && name.back == '»' then
    .str .anonymous name
  else
    name.toName


/-
Override for finding modules to import

FIXME: may not actually update processHeader use
-/
open Lean in
def importModules := Lean.importModules
where importMods : List Import -> StateRefT ImportState IO Unit
  | []    => pure ()
  | i::is => do
    if i.runtimeOnly || (← get).moduleNameSet.contains i.module then
      importMods is
    else do
      importMods is

/-
FIXME: override module searching/loading
-/
open Lean Elab
open System (FilePath) in
def runFrontend (input : String) (filePath : FilePath) : IO Environment := do
  -- make root input context (src, filename, filemap)
  let inputCtx := Parser.mkInputContext input filePath.toString

  -- parses the module...?
  let (header, parserState, messages) ← Parser.parseHeader inputCtx

  -- parses imports from header `processHeader -> importModules`
    -- `importMods -> findOLean -> readModuleData`
    -- `-> (ModuleData, region) -> ImportState -> IO Environment`
  let (env, messages) ← processHeader header default messages inputCtx 0

  let env := env.setMainModule default
  let commandState := Command.mkState env messages default
  let s ← IO.processCommands inputCtx parserState commandState

  let msgs := s.commandState.messages
  if msgs.hasErrors then
    throw $ IO.userError $ "\n\n".intercalate $
      (← msgs.toList.mapM (·.toString)).map String.trim
  else return s.commandState.env

-- end Lean.Elab

open System HVM.Codegen HVM.Codegen in
def main : IO Unit := do
  -- simpleCodeGenRun `simpleCodeGenRun

  -- Compute HVM expression
  -- Lean.setLibsPaths
  let path := "Demo.lean"
  let decl := toNameSafe "add"

  let rulebook ← match codeGen (← runFrontend (← IO.FS.readFile path) path) decl with
  | .error msg => IO.eprintln msg; return ()
  | .ok rulebook => pure rulebook

  dbg_trace s!">> codegen'ed rulebook: {rulebook.toString}"

  -- let args ← IO.appArgs
  -- let input ← IO.FS.readFile args[1]
  -- let output := compile input
  -- IO.FS.writeFile args[2] output

  -- let stt ← match ← contAddr constMap delta false true with
  --   | .error err => IO.eprintln err; return 1
  --   | .ok stt => pure stt

  return ()

#eval main
