{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    ...
  } @ inputs: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShell.${system} = devenv.lib.mkShell {
      inherit inputs pkgs;
      modules = [
        ({pkgs, ...}: {
          enterShell = ''
            rustc --version
          '';

          packages = [
            pkgs.pkg-config
            pkgs.libressl
            pkgs.nodejs_18
            pkgs.worker-build
          ];

          languages.rust.enable = true;
          languages.javascript.enable = true;
        })
      ];
    };
  };
}
