{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "nyaa-bulk";
        version = "0.1.0";

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        src = ./.;
        cargoSha256 = "sha256-MlX7eu91i2qm6W35qgTU4Gr/gn0ymictDkMW//fxm1U=";
      };

      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          nixd
          openssl
          pkg-config
          rust-analyzer
          rustc
        ];
      };
    };
}
