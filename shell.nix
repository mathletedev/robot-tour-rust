{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib }:

let
  pkgsUnstable = import <nixpkgs-unstable> { };
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    pkgsUnstable.probe-rs-tools
    pkgsUnstable.flip-link
    gcc-arm-embedded
  ];
}
