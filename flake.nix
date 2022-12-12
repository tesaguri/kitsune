{
    inputs = {
        flake-utils.url = "github:numtide/flake-utils";
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };
    outputs = { self, flake-utils, nixpkgs, rust-overlay }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {
                    inherit overlays system;
                };
            in
            {
                devShell = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        nodejs
                        postgresql_15
                        redis
                        rust-bin.stable.latest.default
                        sqlite
                        yarn
                    ];
                };
            }
        );
}
