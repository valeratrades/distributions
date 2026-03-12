{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    v-utils.url = "github:valeratrades/.github?ref=v1.4";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, pre-commit-hooks, v-utils, ... }:
    let
      manifest = (nixpkgs.lib.importTOML ./Cargo.toml).package;
      pname = manifest.name;
    in
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = builtins.trace "flake.nix sourced" [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-src" "rust-analyzer" "rust-docs" "rustc-codegen-cranelift-preview" ];
          });
          pre-commit-check = pre-commit-hooks.lib.${system}.run (v-utils.files.preCommit { inherit pkgs; });
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;

          github = v-utils.github {
            inherit pkgs pname;
            lastSupportedVersion = "nightly-2025-10-12";
            langs = [ "rs" ];
            jobs.default = true;
          };
          rs = v-utils.rs { inherit pkgs rust; };
          readme = v-utils.readme-fw { inherit pkgs pname; defaults = true; lastSupportedVersion = "nightly-1.92"; rootDir = ./.; badges = [ "msrv" "loc" "ci" ]; };
        in
        {
          packages =
            let
              rustc = rust;
              cargo = rust;
              rustPlatform = pkgs.makeRustPlatform {
                inherit rustc cargo stdenv;
              };
            in
            {
              default = rustPlatform.buildRustPackage {
                inherit pname;
                version = manifest.version;

                cargoLock = {
                  lockFile = ./Cargo.lock;
                  outputHashes = {
                    "v_utils-2.15.30" = "sha256-zmVE9AF2lfzCFw/OaeH02HCWwNdj4+MvkuGznmc4nLs=";
                  };
                };
                src = pkgs.lib.cleanSource ./.;
              };
            };

          devShells.default = with pkgs; mkShell {
            inherit stdenv;
            shellHook =
              pre-commit-check.shellHook +
              github.shellHook +
              rs.shellHook +
              readme.shellHook +
              ''
                cp -f ${(v-utils.files.treefmt) {inherit pkgs;}} ./.treefmt.toml
              '';
            env = {
              RUST_BACKTRACE = 1;
              RUST_LIB_BACKTRACE = 0;
            };

            packages = [
              mold
              rust
            ] ++ pre-commit-check.enabledPackages ++ github.enabledPackages;
          };
        }
      );
}
