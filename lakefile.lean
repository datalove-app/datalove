import Lake
open Lake DSL

package «datalove» { }

-- @[default_target]
-- lean_exe hvm {
--   root := `Main
-- }

lean_lib «Datalove» {
  -- add library configuration options here
  roots := #[`Datalove]
}

-- lean_lib HVM {
--   -- add library configuration options here
--   roots := #[`HVM]
-- }

-- require mathlib from git
--   "https://github.com/leanprover-community/mathlib4.git"
-- require std from git
--   "https://github.com/leanprover/std4.git" @ "main"
-- require «sql-utils» from git
--   "https://github.com/FWuermse/lean4-sql-utils.git" @ "main"
-- require Lurk from git
--   "vendor/lurk-lean"
require MLIR from "vendor/lean-mlir"
-- require Papyrus from "vendor/lean-papyrus"
-- require Yatima from "vendor/yatima"
-- require YatimaStdLib from git
--   "vendor/yatimastdlib"

-- dev is so not everyone has to build it
meta if get_config? env = some "dev" then
  require «doc-gen4» from git
    "https://github.com/leanprover/doc-gen4" @ "5ab6766eb17a118ed490216305de2a7651e9ebf8"

-- script demo do
--   IO.println "running compile of `Demo.lean`"
--   compileLLVMWasm
