{
  description = "Development shell for layout-rs";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            pkgs = import nixpkgs { inherit system; };
          }
        );
    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              graphviz
            ];

            shellHook = ''
              echo "layout-rs dev shell"
              echo "Using host Rust: $(rustc --version)"
              echo "Graphviz: $(dot -V 2>&1)"
            '';
          };
        }
      );
    };
}
