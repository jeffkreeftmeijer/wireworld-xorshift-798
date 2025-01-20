{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/ae2fb9f1fb5fcf17fb59f25c2a881c170c501d6f";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            lld
            wasm-bindgen-cli
            binaryen
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
