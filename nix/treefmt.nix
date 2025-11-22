{ pkgs, ... }:
let
  rustfmt-config =
    pkgs.writeText "rustfmt.toml" # toml
      ''
        edition = "2024"
        # Put a trailing comma after a block based match arm (non-block arms are not affected)
        match_block_trailing_comma = true
        # Merge multiple derives into a single one.
        merge_derives = true
        # Reorder import and extern crate statements alphabetically in groups (a group is separated by a newline).
        reorder_imports = true
        # Reorder mod declarations alphabetically in group.
        reorder_modules = true
        # Use field initialize shorthand if possible.
        use_field_init_shorthand = true
        # Replace uses of the try! macro by the ? shorthand
        use_try_shorthand = true
      '';
in
{
  projectRootFile = "flake.nix";

  programs = {
    # Rust formatting
    rustfmt.enable = true;

    # Nix formatting
    nixpkgs-fmt.enable = true;
  };

  settings.formatter = {
    nixpkgs-fmt = {
      excludes = [ ];
    };

    rustfmt = {
      options = [
        "--config-path"
        "${rustfmt-config}"
      ];
      excludes = [
        "src/action/linux/selinux/*"
      ];
    };
  };
}
