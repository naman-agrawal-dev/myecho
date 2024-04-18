{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "myecho";
        };
        nativeBuildInputs = [ pkgs.makeWrapper ];
        installPhase = ''
          cp target/release/myecho $out/bin/
        '';
        postInstall = ''
          wrapProgram "$out/bin/myecho""
        '';
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rust-analyzer
            rustPackages.clippy
            rustc
            rustfmt
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
