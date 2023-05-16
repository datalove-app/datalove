import Lake
open Lake DSL

package «datalove» {
  -- add package configuration options here
}

@[default_target]
lean_lib «Datalove» {
  -- add library configuration options here
  roots := #[`Datalove]
}

lean_lib HVM {
  -- add library configuration options here
  roots := #[`HVM]
}

-- require mathlib from git
--   "https://github.com/leanprover-community/mathlib4.git"
require std from git
  "https://github.com/leanprover/std4.git" @ "main"
-- require Lurk from git
--   "vendor/lurk-lean"
-- require Yatima from git
--   "vendor/yatima"
-- require YatimaStdLib from git
--   "vendor/yatimastdlib"

-- dev is so not everyone has to build it
meta if get_config? env = some "dev" then
  require «doc-gen4» from git "https://github.com/leanprover/doc-gen4" @ "main"

-- script demo do
--   IO.println "running compile of `Demo.lean`"
