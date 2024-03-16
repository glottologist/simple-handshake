{
  description = "Flake for Simple Handshake app";

  inputs = {
    nixpkgs.url = "github:glottologist/nixpkgs/master";
    fenix.url = "github:nix-community/fenix";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = [
      "https://tweag-jupyter.cachix.org"
      "https://devenv.cachix.org"
    ];
    extra-trusted-public-keys = [
      "tweag-jupyter.cachix.org-1:UtNH4Zs6hVUFpFBTLaA4ejYavPo5EFFqgd7G7FxGW9g="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
    ];
  };

  outputs = inputs @ {
    flake-parts,
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: rec {
        packages = rec {
          default = pkgs.callPackage ./default.nix {inherit pkgs;};
        };
        apps = {
          rustApp = flake-utils.lib.mkApp {drv = self'.packages.${system}.rust;};
        };

        devenv.shells.default = devenv.shells.rust;
        devenv.shells.rust = {
          name = "Shell for simple handshake";
          env.GREET = "devenv for the simple handshake";
          packages = with pkgs; [
            git
            solana-validator
          ];
          enterShell = ''
            git --version
            rustc --version
            cargo --version
            solana --version
          '';
          languages = {
            rust.enable = true;
            rust.channel = "nightly";
          };
          difftastic.enable = true;
          pre-commit.hooks = {
            alejandra.enable = true;
            commitizen.enable = true;
            cargo-check.enable = true;
            clippy.enable = true;
          };
        };
      };
    };
}
