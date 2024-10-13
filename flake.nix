{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-parts.url = "github:hercules-ci/flake-parts";
		agenix-shell.url = "github:aciceri/agenix-shell";
	};

	outputs =
		inputs@{
			self,
			nixpkgs,
			flake-parts,
			agenix-shell,
			...
		}:
	flake-parts.lib.mkFlake { inherit inputs; } {
		systems = nixpkgs.lib.systems.flakeExposed;

		imports = [
# agenix-shell.flakeModules.default
		];

# agenix-shell = {
#   secrets = {
#     foo.file = ./secrets/foo.age;
#   };
# };

		perSystem =
		{
			pkgs,
			config,
			lib,
			...
		}:
		let
			inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
		toolchain = pkgs.rustPlatform;
		in
		{
			devShells.default = pkgs.mkShell {
				packages = with pkgs; [
					openssl
					(with toolchain;
					[
					 rustc 
					 cargo
					])
					clippy
					rustfmt
					rust-analyzer-unwrapped
					libiconv
				] ++ lib.optionals stdenv.isDarwin [
				darwin.libobjc  
				SystemConfiguration
				];
				RUST_SRC_PATH = "${toolchain.rustLibSrc}";
			};
		};
	};
}
