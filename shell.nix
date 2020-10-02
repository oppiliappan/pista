{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "rust-env";
  nativeBuildInputs = with pkgs; [
    rustc cargo
  ];
  buildInputs = with pkgs; [ openssl pkgconfig ];

  RUST_BACKTRACE = 1;
}
