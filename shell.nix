{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShellNoCC {
    nativeBuildInputs = with pkgs.buildPackages; [
			sqlx-cli
			pkg-config
			openssl
		];
}

