{
  pkgs ? import (fetchGit {
    url = https://github.com/NixOS/nixpkgs-channels;
    ref = "nixos-unstable";
  }) {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
  ];
}
