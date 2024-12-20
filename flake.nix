{
  description = "Egui transition animation";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixkgs";
    };
    crate2nix.url = "github:nix-community/crate2nix";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    crate2nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        crate2nix-tools = pkgs.callPackage "${crate2nix}/tools.nix" {};

        generatedCargoNix = crate2nix-tools.generatedCargoNix {
            name = "egui_transition_example";
            src = ./.;
        };

        cargoNix = pkgs.callPackage "${generatedCargoNix}/default.nix" {
        };

        # TODO: Is anything superflous here?
        eguiLibs = with pkgs; [
          wayland
          libxkbcommon
          libGL
          libGLU
        ] ++ (with pkgs.xorg; [
          libX11
          libxcb
          libXcursor
          libXrandr
          libXi
          pkg-config
        ]);
      in {
        packages.default = cargoNix.rootCrate.build.override {
            runTests = true;
        };

        /*pkgs.symlinkJoin {
          name = "egui_transition_example";
          paths = [ cargoNix.rootCrate.build ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/egui_transition_animation \
              --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath eguiLibs}
              mv $out/bin/egui_transition_animation $out/bin/egui_transition_example
          '';
          };*/
        #;

        devShell = pkgs.mkShell rec {
          nativeBuildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" "cargo" "rustc" "clippy" "rust-analyzer" ];
            })
            pkgs.gcc
          ] ++ eguiLibs;

          shellHook = ''
              export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${pkgs.lib.makeLibraryPath eguiLibs}
          '';

          RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
          }}/lib/rustlib/src/rust/library";


          buildInputs = with pkgs; [
            openssl.dev
            glib.dev
            pkg-config
          ];
        };
      }
    );
}
