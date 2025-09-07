{ self, ... }:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.yt-dlp.yt-dli;

  listOrSingleton =
    type:
    lib.types.coercedTo (lib.types.either (lib.types.listOf type) type) lib.toList (
      lib.types.listOf type
    );
in
{
  options.programs.yt-dlp.yt-dli = {
    enable = lib.mkEnableOption "yt-dli";
    package = lib.mkPackageOption self.packages.${pkgs.system} "yt-dli" {
      default = "yt-dli";
    };

    profiles = lib.mkOption {
      type = lib.types.attrsOf (
        lib.types.submodule {
          options = {
            references = lib.mkOption {
              type = listOrSingleton lib.types.str;
              default = [ ];
              example = "video";
              description = ''
                Other profile(s) that this profile should reference and load.
              '';
            };

            # Adapted: https://github.com/nix-community/home-manager/blob/f56bf065f9abedc7bc15e1f2454aa5c8edabaacf/modules/programs/yt-dlp.nix#L29-L71
            settings = lib.mkOption {
              type =
                with lib.types;
                attrsOf (oneOf [
                  bool
                  int
                  str
                ]);
              default = { };
              example = lib.literalExpression ''
                {
                  embed-thumbnail = true;
                  embed-subs = true;
                  sub-langs = "all";
                  downloader = "aria2c";
                  downloader-args = "aria2c:'-c -x8 -s8 -k1M'";
                }
              '';
              description = ''
                Configuration written to
                {file}`$XDG_CONFIG_HOME/yt-dli/<profile>`.

                Options must be specified in their "long form", for
                example, `update = true;` instead of `U = true;`.
                Short options can be specified in the `extraConfig` option.
                See <https://github.com/yt-dlp/yt-dlp#configuration>
                for explanation about possible values.
              '';
            };
            extraConfig = lib.mkOption {
              type = lib.types.lines;
              default = "";
              example = ''
                --update
                -F
              '';
              description = ''
                Extra configuration to add to
                {file}`$XDG_CONFIG_HOME/yt-dli/<profile>`.
              '';
            };
          };
        }
      );
      default = { };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = with cfg; [ package ];

    xdg.configFile =
      let
        renderSettings = lib.mapAttrsToList (
          name: value:
          if builtins.isBool value then
            if value then "--${name}" else "--no-${name}"
          else
            "--${name} ${toString value}"
        );
      in
      lib.mapAttrs' (
        name: profile:
        lib.nameValuePair "yt-dli/${name}" {
          text =
            builtins.concatStringsSep "\n" (
              lib.remove "" (
                renderSettings profile.settings
                ++ (map (
                  reference: "--config-locations ${lib.escapeShellArg config.xdg.configHome}/yt-dli/${reference}"
                ) profile.references)
                ++ [ profile.extraConfig ]
              )
            )
            + "\n";
        }
      ) cfg.profiles;
  };
}
