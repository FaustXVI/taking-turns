{ pkgs, self, system, craneLib, cargoArtifacts, commonArgs, src, advisory-db, ... }:
{
  # Each target can be ran separately using `nix build .#checks.<system>.<target>`, e.g. `nix build .#checks.x86_64-linux.tests`

  # Run clippy checks
  clippy = craneLib.cargoClippy (commonArgs // {
    inherit cargoArtifacts;
    cargoClippyExtraArgs = "--all-targets -- --deny warnings";
  });

  # Run check regarding *.rs file formatting
  fmt = craneLib.cargoFmt {
    inherit src;
  };

  # Run check regarding *.toml file formatting
  toml-fmt = craneLib.taploFmt {
    src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
  };

  # Build the documentation for the local package and all dependencies
  doc = craneLib.cargoDoc (commonArgs // {
    inherit cargoArtifacts;
  });

  # Run check with `cargo-audit` (e.g. security vulns).
  audit = craneLib.cargoAudit {
    inherit src advisory-db;
    cargoAuditExtraArgs = ""; # Removing the default `--ignore yanked`, as we have a .cargo/audit.toml that controls that
  };

  # Run check with `cargo-deny` and look after dependencies (see deny.toml).
  deny = craneLib.cargoDeny {
    inherit src;
  };

}

