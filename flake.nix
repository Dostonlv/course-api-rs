{
  inputs = {
    nixpkgs.url = "git+https://git.oss.uzinfocom.uz/xinux/nixpkgs?ref=nixos-26.05&shallow=1";
    # flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    postgres = pkgs.postgresql_18;
    run-postgres = pkgs.writeShellApplication {
      name = "run-postgres";
      runtimeInputs = [
        postgres
        pkgs.git
      ];
      text = ''
        PROJECT_ROOT="$(git rev-parse --show-toplevel)"
        RUNTIME="$PROJECT_ROOT/.runtime"
        PG_DATA="$RUNTIME/pg_data"

        if [[ ! -d $PG_DATA ]]; then
          echo "=================================================="
          echo "No PG_DATA folder, initializing it at \"$PG_DATA\""
          echo "=================================================="
          mkdir -p "$PG_DATA"
          initdb -D "$PG_DATA" -U postgres --pwfile="${pkgs.writeText "default_postgres_password" "postgres"}"
          echo "=================================================="
          echo "Created user 'postgres' with a password 'postgres'"
          echo "So you can connect to it using 'pgadmin4' app"
          echo "=================================================="
        fi

        postgres -D "$PG_DATA" -k "$PG_DATA"
      '';
    };
  in {
    devShell.x86_64-linux = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = with pkgs; [
        openssl
        openssl.dev
      ];

      packages = with pkgs; [
        rustc
        cargo
        clippy
        sqlx-cli

        run-postgres
      ];

      shellHook = ''
        zsh
      '';
    };
  };
}
