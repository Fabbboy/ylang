# Sable

This repository contains the sources for the Sable programming language.

## Tree-sitter grammar

A basic [Tree-sitter](https://tree-sitter.github.io/) grammar lives in
`tree-sitter-sable`. After running `npm install` inside this directory you can
run `tree-sitter generate` to build the parser. Editors like VSCode can use
this grammar for syntax highlighting.

### Using with Neovim

If you use [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)
you can add the Sable grammar as a custom parser. Place something like the
following in your `init.lua` and adjust the `url` path to where this repository
is cloned:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
parser_config.sable = {
  install_info = {
    url = "/path/to/tree-sitter-sable",
    files = {"src/parser.c"},
    generate_requires_npm = false,
  },
  filetype = "sable",
}

require("nvim-treesitter.configs").setup {
  highlight = { enable = true },
}
```

Restart Neovim and run `:TSInstall sable` to build the parser.  If you prefer,
you can also install the grammar on the fly without editing your config by
executing `:TSInstallFromGrammar /path/to/tree-sitter-sable`.
