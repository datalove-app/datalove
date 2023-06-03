import Lean

-- import HVM.Codegen.Codegen
-- import HVM.Compiler.Compiler
-- import Yatima.ContAddr.ContAddr
import Datalove.Lib
import Datalove.Kernel.Bootstrap

namespace Datalove


-- #eval Lean.getBuildDir
-- #eval compileWasm



-- open System Lean.IR
-- def compileC : IO Unit := do
--   let input := "Demo.lean"
--   let output := "out/Demo.c"
--   let decl := toNameSafe "add"

--   -- Lean.setLibsPaths
--   let src ← runFrontend (← IO.FS.readFile input) input
--   let file ← match emitC src decl with
--   | .error msg => IO.eprintln msg; return ()
--   | .ok file => pure file

--   -- dbg_trace s!">> C file: {file}"

--   IO.FS.writeFile output file

-- #eval compileC



-- open System HVM.Codegen in
-- def codegenDemo : IO Unit := do
--   let input := "out/Demo.lean"
--   let output := "out/Demo.hvm"
--   let decl := toNameSafe "add"

--   -- Lean.setLibsPaths
--   let src ← runFrontend (← IO.FS.readFile input) input
--   let rulebook ← match codeGen src decl with
--   | .error msg => IO.eprintln msg; return ()
--   | .ok rulebook => pure rulebook

--   dbg_trace s!">> codegen'ed rulebook: {rulebook.toString}"

--   -- let args ← IO.appArgs
--   -- let input ← IO.FS.readFile args[1]
--   -- let output := compile input
--   -- IO.FS.writeFile output rulebook.toString
--   return ()

-- #eval codegenDemo



-- open System Lean.Compiler.LCNF HVM.Grin in
-- def compileDemo : IO Unit := do
--   let input := "out/Demo.lean"
--   let output := "out/Demo.hvm"
--   let decl := toNameSafe "add"

--   -- Lean.setLibsPaths
--   let src ← runFrontend (← IO.FS.readFile input) input
--   let bindings ← match compile src decl with
--   | .error msg => IO.eprintln msg; return ()
--   | .ok bindings => pure bindings

--   dbg_trace s!">> codegen'ed bindings: {bindings.toString}"

--   -- let args ← IO.appArgs
--   -- let input ← IO.FS.readFile args[1]
--   -- let output := compile input
--   -- IO.FS.writeFile output bindings.toString
--   return ()

-- #eval compileDemo

end Datalove
