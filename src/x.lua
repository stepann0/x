local function call_x(arg)
  local code, out = 101, ""
  pcall(function ()
    vim.system({'x', arg}, { text = true }, function(obj)
      code = obj.code
      out = obj.stdout:sub(0, string.len(obj.stdout)-1)
    end):wait()
  end)

  if code == 0 then
    return true, out
  end
  return false, ""
end

vim.api.nvim_create_user_command("X",
  -- works only with `viw` selection
  function(opts)
    local line = vim.api.nvim_get_current_line()
    local v_start = vim.fn.getpos("'<")[3]
    local v_end = vim.fn.getpos("'>")[3]

    local number_to_replace = line:sub(v_start, v_end)
    local expr = number_to_replace..opts.fargs[1]
    local replace = number_to_replace

    local ok, res = call_x(expr)
    if ok then replace = res end

    local new_line = line:sub(0, v_start-1) .. replace .. line:sub(v_end+1)
    vim.api.nvim_set_current_line(new_line)
  end,
  { nargs = 1, range = true})

vim.keymap.set("n", "<leader>xb", "viw:X .b<CR>w") -- useful keymap
