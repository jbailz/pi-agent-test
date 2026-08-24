{
  description = "Tiny Rust Space Invaders for native and browser play";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            lld
            pkg-config
            curl
            python3
          ] ++ lib.optionals stdenv.isLinux [
            alsa-lib
            libGL
            xorg.libX11
            xorg.libXi
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXinerama
          ];
        };
      });
    };
}
