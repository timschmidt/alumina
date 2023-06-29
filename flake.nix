{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        inherit (pkgs) rust-bin mkShell rust-analyzer pkg-config glib gtk3;
        inherit (rust-bin) fromRustupToolchainFile;

        rust = fromRustupToolchainFile ./rust-toolchain;
      in {
        devShells.default = mkShell {
          buildInputs = [ rust rust-analyzer glib gtk3 ];
          nativeBuildInputs = [ pkg-config ];
        };
      });
}
