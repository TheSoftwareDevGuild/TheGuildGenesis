{
  description = "The Guild Genesis - A peer-run organization for software developers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Use pre-built binaries only - avoid LLVM trap
            nodejs
            nodePackages.npm
            nodePackages.pnpm
            rustc
            cargo
            postgresql_14
            git
            curl
            jq
            just
          ];

          shellHook = ''
            echo "🔧 The Guild Genesis Development Environment"
            echo "📦 Available commands:"
            echo "  dev          - Start both frontend and backend"
            echo "  dev-frontend - Start frontend only"
            echo "  dev-backend  - Start backend only"
            echo "  db-start     - Start PostgreSQL database"
            echo "  db-stop      - Stop PostgreSQL database"
            echo "  db-setup     - Set up database with migrations"
            echo "  db-reset     - Reset database completely"
            echo "  install-all  - Install all dependencies"
            echo ""
            echo "🚀 Run 'just <command>' to execute scripts"
            echo ""
            echo "💡 Database uses system PostgreSQL to avoid LLVM builds"
          '';

          # Set environment variables
          DATABASE_URL = "postgres://guild_user:guild_password@localhost:5432/guild_genesis";
          RUST_LOG = "debug";
        };
      });
}

