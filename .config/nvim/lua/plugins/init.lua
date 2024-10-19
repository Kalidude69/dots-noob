return {
  {
    "stevearc/conform.nvim",
    -- event = 'BufWritePre', -- uncomment for format on save
    opts = require "configs.conform",
  },

  -- These are some examples, uncomment them if you want to see them work!
  {
    "neovim/nvim-lspconfig",
    config = function()
      require "configs.lspconfig"
    end,
  },

  -- {
  -- 	"nvim-treesitter/nvim-treesitter",
  -- 	opts = {
  -- 		ensure_installed = {
  -- 			"vim", "lua", "vimdoc",
  --      "html", "css"
  -- 		},
  -- 	},
nvdash = {
      load_on_startup = true,

      header = {
"██╗    ██╗ █████╗ ███████╗███████╗██╗   ██╗██████╗ ",
"██║    ██║██╔══██╗██╔════╝██╔════╝██║   ██║██╔══██╗ ",
"██║ █╗ ██║███████║███████╗███████╗██║   ██║██████╔╝ ",
"██║███╗██║██╔══██║╚════██║╚════██║██║   ██║██╔═══╝  ",
"╚███╔███╔╝██║  ██║███████║███████║╚██████╔╝██║      ",
"╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚══════╝ ╚═════╝ ╚═╝       ",
      },

      buttons = {
        { "  Find File", "Spc f f", "Telescope find_files" },
        { "󰈚  Recent Files", "Spc f o", "Telescope oldfiles" },
        { "󰈭  Find Word", "Spc f w", "Telescope live_grep" },
        { "  Bookmarks", "Spc m a", "Telescope marks" },
        { "  Themes", "Spc t h", "Telescope themes" },
        { "  Mappings", "Spc c h", "NvCheatsheet" },
      },
    },
}
