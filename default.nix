let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage rec {
	pname = "pitr_util";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "18hp0ad4h1r99pxr3k8n2bh6mfmyvp1zgqq2g8jjxj0q6agi5isj";
}
