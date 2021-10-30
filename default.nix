let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage rec {
	pname = "pitr_util";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "19asss043sb4in27z49x444d4i9p2xalkvg1b6kjh7956n504inm";
}
