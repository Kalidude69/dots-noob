   local M = {}

   M.plugins = {
     { -- Dashboard
       'goolord/alpha-nvim',
       requires = { 'kyazdani42/nvim-web-devicons' },
       config = function()
         local alpha = require 'alpha'
         local startify = require 'alpha.themes.startify'

         startify.section.header.val = {
           "███╗   ██╗██╗   ██╗██╗███╗   ███╗",
           "████╗  ██║██║   ██║██║████╗ ████║",
           "██╔██╗ ██║██║   ██║██║██╔████╔██║",
           "██║╚██╗██║╚██╗ ██╔╝██║██║╚██╔╝██║",
           "██║ ╚████║ ╚████╔╝ ██║██║ ╚═╝ ██║",
           "╚═╝  ╚═══╝  ╚═══╝  ╚═╝╚═╝     ╚═╝",
         }

         alpha.setup(startify.config)
       end,
     }
   }

   return M
