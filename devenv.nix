{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [pkgs.git pkgs.xxd pkgs.libudev-zero];

  # https://devenv.sh/languages/
  languages.cplusplus.enable = true;
  languages.rust.enable = true;
  languages.rust.channel = "nightly";

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;
}
