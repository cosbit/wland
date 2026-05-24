{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    pyproject-nix = {
      url = "github:pyproject-nix/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    uv2nix = {
      url = "github:pyproject-nix/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pyproject-build-systems = {
      url = "github:pyproject-nix/build-system-pkgs";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.uv2nix.follows = "uv2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, naersk, pyproject-nix, uv2nix, pyproject-build-systems, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;
        pkgs = import nixpkgs { inherit system; };
        repoRoot = ./.;
        cargoInputs = with pkgs; [
          cargo
          coreutils
          stdenv.cc
          rustc
          rustfmt
          rustPackages.clippy
        ];

        mkCargoApp = { name, subcommand }:
          pkgs.writeShellApplication {
            inherit name;
            runtimeInputs = cargoInputs;
            text = ''
              set -euo pipefail
              workdir="$(mktemp -d)"
              cp -R ${repoRoot}/. "$workdir/"
              cd "$workdir"
              exec cargo ${subcommand} "$@"
            '';
          };

        compileApp = mkCargoApp {
          name = "wland-compile";
          subcommand = "build";
        };
        compileAppDef = utils.lib.mkApp { drv = compileApp; };

        workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = repoRoot; };
        overlay = workspace.mkPyprojectOverlay {
          sourcePreference = "wheel";
        };
        python = lib.head (pyproject-nix.lib.util.filterPythonInterpreters {
          inherit (workspace) requires-python;
          inherit (pkgs) pythonInterpreters;
        });
        pythonBase = pkgs.callPackage pyproject-nix.build.packages {
          inherit python;
        };
        pythonSet = pythonBase.overrideScope (
          lib.composeManyExtensions [
            pyproject-build-systems.overlays.default
            overlay
          ]
        );
        testVenv = pythonSet.mkVirtualEnv "wland-test-env" {
          wland = [ "test" ];
        };
        pythonTestApp = pkgs.writeShellApplication {
          name = "wland-test";
          runtimeInputs = cargoInputs ++ [ testVenv ];
          text = ''
            set -euo pipefail
            workdir="$(mktemp -d)"
            cp -R ${repoRoot}/. "$workdir/"
            cd "$workdir"
            cargo build
            export WLAND_BIN="$workdir/target/debug/wland"
            exec "${testVenv}/bin/python" -m pytest tests/units/ "$@"
          '';
        };
        pythonTestAppDef = utils.lib.mkApp { drv = pythonTestApp; };

        devShell = pkgs.mkShell {
          buildInputs = cargoInputs ++ [ pkgs.pre-commit pkgs.uv pythonSet.python.interpreter ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          shellHook = ''
            export UV_NO_SYNC=1
            export UV_PYTHON=${pythonSet.python.interpreter}
            export UV_PYTHON_DOWNLOADS=never
          '';
        };
      in
      {
        apps.default = compileAppDef;
        apps.compile = compileAppDef;
        apps.test = pythonTestAppDef;
        defaultApp = compileAppDef;

        devShells.default = devShell;
        devShell = devShell;
      }
    );
}
