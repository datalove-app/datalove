{
  description = "Datalove";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    lean4 = {
      # url = "github:HamelinDavid/lean4";
      url = "github:leanprover/lean4";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    lake = {
      url = "github:leanprover/lake";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        # lean.follows = "lean4";
      };
    };

    # std4 = {
    #   url = "github:leanprover/std4";
    #   # url = "vendor/std4";
    #   flake = false;
    # };
    # yatima = {
    #   url = "github:lurk-lab/yatima";
    #   flake = false;
    # };

    # lightdata.lean.url = "github:lurk-lab/lightdata.lean";
    # lurk.lean.url = "github:lurk-lab/lurk.lean";
    # yatima.url = "github:lurk-lab/yatima"
    # yatimastdlib.lean.url = "github:lurk-lab/yatimastdlib.lean";
    # wasm.lean.url = "github:lurk-lab/wasm.lean";

    # typst.url = "github:typst/typst";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    lean4,
    lake,
    # std4,
    # yatima,
    ...
  }:
    # let
    #   supportedSystems = [
    #     # "aarch64-linux"
    #     "aarch64-darwin"
    #     "i686-linux"
    #     "x86_64-darwin"
    #     "x86_64-linux"
    #   ];
    # in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          # config.allowUnfree = true;
        };
        leanPkgs = lean4.packages.${system};
        lakePkgs = lake.packages.${system};

        # deps
        # lurk = leanPkgs.buildLeanPackage {
        #   name = "Lurk";
        #   src = ./vendor/lurk-lean;
        # };
        # Std = leanPkgs.buildLeanPackage {
        #   name = "Std";
        #   src = std4;
        # };
        # Yatima = leanPkgs.buildLeanPackage {
        #   name = "Yatima";
        #   src = yatima;
        # };

        pkg = leanPkgs.buildLeanPackage {
          name = "Datalove";
          # roots = [ "Datalove" ];
          src = ./.;
          deps = [ leanPkgs.Init leanPkgs.Lean ];
        };
      in {
        packages = pkg // {
          inherit (leanPkgs) lean;
          inherit (lakePkgs) lake;
        };
        defaultPackage = pkg.modRoot;

        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [ leanPkgs.lean-all lakePkgs.cli ];
          shellHook = ''
            export PATH=${pkgs.lib.makeBinPath buildInputs}:$PATH
          '';
          # LEAN_PATH = "./src:./test";
          # LEAN_SRC_PATH = "./src:./test";
        };
      });
}
