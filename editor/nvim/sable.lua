local util = require('lspconfig.util')

return {
  default_config = {
    cmd = { 'sable-lsp' },
    filetypes = { 'sable' },
    root_dir = function(fname)
      return util.root_pattern('Cargo.toml', '.git')(fname) or util.path.dirname(fname)
    end,
    single_file_support = true,
  },
  docs = {
    description = [[
Configuration for the Sable language server.
    ]],
  },
}
