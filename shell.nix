{ pkgs ? import <nixpkgs> {} }:
	pkgs.mkShell {
		nativeBuildInputs = [
			pkgs.pkg-config
			pkgs.lua52Packages.lua
			pkgs.rustup
			pkgs.SDL2
	];
}
