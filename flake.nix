{
  inputs = {
    nixpkgs.url = "github:jordanisaacs/nixpkgs/aoc-cli-init";
    rust-overlay.url = "github:oxalica/rust-overlay";
    neovim-flake.url = "github:jordanisaacs/neovim-flake";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    neovim-flake,
    crate2nix,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [
      rust-overlay.overlays.default
      neovim-flake.overlays.default
      (self: super: let
        rust = super.rust-bin.stable.latest.default;
      in {
        rustc = rust;
        cargo = rust;
        neovim = self.neovimBuilder {
          config = {
            vim.lsp = {
              enable = true;
              lightbulb.enable = true;
              lspSignature.enable = true;
              nvimCodeActionMenu.enable = true;
              formatOnSave = true;
              rust = {
                enable = true;
                rustAnalyzerOpts = "";
              };
              nix.enable = true;
            };
            vim.statusline.lualine = {
              enable = true;
              theme = "onedark";
            };
            vim.visuals = {
              enable = true;
              nvimWebDevicons.enable = true;
              lspkind.enable = true;
              indentBlankline = {
                enable = true;
                fillChar = "";
                eolChar = "";
                showCurrContext = true;
              };
              cursorWordline = {
                enable = true;
                lineTimeout = 0;
              };
            };

            vim.theme = {
              enable = true;
              name = "onedark";
              style = "darker";
            };
            vim.autopairs.enable = true;
            vim.autocomplete = {
              enable = true;
              type = "nvim-cmp";
            };
            vim.filetree.nvimTreeLua.enable = true;
            vim.tabline.nvimBufferline.enable = true;
            vim.telescope = {
              enable = true;
            };
            vim.markdown = {
              enable = true;
              glow.enable = true;
            };
            vim.treesitter = {
              enable = true;
              autotagHtml = true;
              context.enable = true;
            };
            vim.keys = {
              enable = true;
              whichKey.enable = true;
            };
            vim.git = {
              enable = true;
              gitsigns.enable = true;
            };
          };
        };
      })
    ];
    pkgs = import nixpkgs {
      inherit system overlays;
    };

    inherit
      (import "${crate2nix}/tools.nix" {inherit pkgs;})
      generatedCargoNix
      ;

    name = "advent-of-code";
    pkg =
      (
        import
        (generatedCargoNix {
          inherit name;
          src = ./.;
        })
        {inherit pkgs;}
      )
      .workspaceMembers
      .client
      .build;

    nativeBuildInputs = with pkgs; [
      rustc

      cargo
      cargo-edit
      cargo-audit
      cargo-tarpaulin
      clippy
    ];

    alias = "aoc -s /home/jd/Documents/dev/advent-of-code/.adventofcode.session";
  in
    with pkgs; {
      packages.${system} = {
        ${name} = pkg;
        default = pkg;
      };
      devShells.${system}.default = mkShell {
        nativeBuildInputs =
          nativeBuildInputs
          ++ [
            neovim
            pkgs.aoc-cli
          ];
        shellHook = ''
          if [ -z ''${IS_DIRENV} ];
          then
            alias aoc="${alias}";
          else
            export_alias aoc '${alias} $@'
          fi
          echo HI
        '';
      };
    };
}
