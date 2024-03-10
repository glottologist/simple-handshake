{pkgs ? import <nixpkgs> {}}: let
  lib = pkgs.lib;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = "mixer";
    version = "0.1.0";
    src = pkgs.lib.cleanSource ./.;
    # Specify the binary that will be installed
    cargoBinName = pname;

    buildInputs = [pkgs.openssl];

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    # The package manager needs to know the SHA-256 hash of your dependencies
    cargoSha256 = "565hrIUXGuOHoxiUEh5CsgUWgD3nUTNKGwZ2b+4FWog=";

    meta = with pkgs.stdenv.lib; {
      homepage = "https://github.com/glottologist/simple-handshake";
      description = "A simple node handshake app.";
      licenses = lib.licence.mit;
      maintainers = with lib.maintainers; [
        {
          name = "Jason Ridgway-Taylor";
          email = "jason@glottologist.co.uk";
          github = "glottologist";
        }
      ];
    };
  }
