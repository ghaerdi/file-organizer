{ pkgs, lib, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  languages.rust = {
    # Pin to the host's default rustup toolchain (1.96.0) so devenv builds and
    # plain `cargo clippy` use the same rustc and share target/ without E0514.
    channel = "1.96.0";
    components = [ "rustc" "cargo" "rust-std" "clippy" "rustfmt" "rust-analyzer" ];
    lsp.enable = true;
  };

  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy = {
      enable = true;
      # Use the project's pinned clippy (with pedantic+nursery lints denied in
      # Cargo.toml [lints.clippy]) rather than Nixpkgs' default toolchain.
      entry = "${pkgs.rustup}/bin/cargo-clippy clippy -- -D warnings";
    };
    commitizen.enable = true;
  };

  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  # https://devenv.sh/basics/
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # See full reference at https://devenv.sh/reference/options/
}
