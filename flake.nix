{
  description = "Datalove";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    lean.url = "github:leanprover/lean4";
    lake.url = "github:leanprover/lake";
    # std4.url = "github:leanprover/std4";
    flake-utils.url = "github:numtide/flake-utils";

    # lightdata.lean.url = "github:lurk-lab/lightdata.lean";
    # lurk.lean.url = "github:lurk-lab/lurk.lean";
    # yatima.url = "github:lurk-lab/yatima"
    # yatimastdlib.lean.url = "github:lurk-lab/yatimastdlib.lean";
    # wasm.lean.url = "github:lurk-lab/wasm.lean";

    # typst.url = "github:typst/typst";
  };

  outputs = { self, nixpkgs, lean, lake, flake-utils }:
    let
      supportedSystems = [
        # "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        packageName = "Datalove";
        leanPkgs = lean.packages.${system};
        lakePkgs = lake.packages.${system};
        # deps
        # lurk = leanPkgs.buildLeanPackage {
        #   name = "Lurk";
        #   src = ./vendor/lurk-lean;
        # };
        # std4 = leanPkgs.buildLeanPackage {
        #   name = "Std";
        #   src = ./vendor/std4/std;
        # };
        # yatima = leanPkgs.buildLeanPackage {
        #   name = "Yatima";
        #   src = ./vendor/yatima;
        #   # deps = [ lurk ];
        # };

        pkg = leanPkgs.buildLeanPackage {
          name = packageName;
          src = ./.;
          # deps = [ lean.Lean std4 ];
        };
      in {
        packages = pkg // {
          # ${name} = datalove.sharedLib;
          # inherit (datalove) lean-package print-paths;
          inherit (leanPkgs) lean-all;
          inherit (lakePkgs) lake;
        };

        defaultPackage = pkg.modRoot;
        # defaultPackage = self.packages.${system}.datalove;
        # devShell = pkg.mkShell {
        #   buildInputs = with pkg; [
        #     leanPkgs.lean-all
        #     lakePkgs.lake
        #   ];
        #   LEAN_PATH = "./src:./test";
        #   LEAN_SRC_PATH = "./src:./test";
        # };
      });
}
