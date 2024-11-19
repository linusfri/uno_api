{
    description = "A flake for building a simple rust app";

    inputs = {
      nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
      flake-utils.url = "github:numtide/flake-utils";
    };
  
    outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem(system:
      let 
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages = rec {
          uno-api = pkgs.rustPlatform.buildRustPackage {
            pname = "uno_api";
            version = "0.1";
            buildInputs = with pkgs; [
              libmysqlclient
            ];
            postInstall = ''
              cp -r ./migrations $out/bin/migrations
            '';
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
            nativeBuildInputs = [
              pkgs.pkg-config
            ];
          };

          default = uno-api;
        };
      }
    );
}
