local is_unix = vim.fn.has 'unix' == 1
local is_win = vim.fn.has 'win32' == 1
local is_wsl = is_unix
  and vim.fn.stridx(vim.fn.system 'uname -a', 'WSL2') >= 0

local function parse_primary_solution()
  local file_path = 'solutions/src/lib.rs'

  -- ファイルの内容を読み込む
  local file = io.open(file_path, 'r')
  if not file then
    print('Failed to open file: ' .. file_path)
    return
  end
  local content = file:read '*all'
  file:close()

  -- 最後の define_solutions! 行を見つける
  local define_solutions_line =
    content:match 'define_solutions!%[.-%]%s*;?%s*$'

  if not define_solutions_line then
    print 'define_solutions! not found'
    return
  end

  -- マクロの引数を抽出
  local solutions = define_solutions_line:match '%[(.+)%]'
  if not solutions then
    print 'No solutions found in define_solutions!'
    return
  end

  -- 最後の要素を取得
  local last_solution = solutions:match '([^ ,]+)[%s,]*$'
  if not last_solution then
    print 'No valid solution found'
    return
  end

  -- snake_case に変換
  local snake_case = last_solution
    :gsub('\n', '')
    :gsub('(%u)', function(c)
      return '_' .. c:lower()
    end)
    :gsub('^_', '')
  snake_case = snake_case:gsub('_solution$', '')

  return snake_case
end

local clipboard
if is_unix and vim.fn.executable 'wl-copy' ~= 0 then
  clipboard = 'wl-copy'
elseif is_unix and vim.fn.executable 'xsel' ~= 0 then
  clipboard = 'xsel -bi'
elseif is_win or is_wsl then
  if vim.fn.executable 'win32yank.exe' ~= 0 then
    clipboard = 'win32yank.exe -i'
  else
    vim.notify(
      '[vimrc] win32yank not found. fallback to clip.exe, but it may collapse on non-ASCII chars',
      vim.log.levels.WARN
    )
    clipboard = 'clip.exe'
  end
end

local function make_file_path(dir_override, kind, id)
  if id < 0 then
    return
  end

  local components = {}

  if dir_override then
    table.insert(components, dir_override)
  else
    table.insert(components, 'testing')
    if kind == 'in' then
      table.insert(components, 'in')
    else
      table.insert(components, 'out')
      table.insert(components, parse_primary_solution())
    end
  end

  local file_name =
    string.format('%04d.txt%s', id, kind == 'err' and '.stderr' or '')
  table.insert(components, file_name)

  return vim.fn.expand(table.concat(components, '/'))
end

local function open_result(dir, id)
  if id < 0 then
    return
  end

  local bufname = vim.fn.expand '%:t' --[[@as string]]
  if not vim.endswith(bufname, '.txt.stderr') then
    vim.cmd 'tabnew'
  end

  vim.cmd(string.format('edit %s', make_file_path(dir, 'err', id)))
end

local function parse_bufname()
  local dir = vim.fn.expand '%:h'
  local bufname = vim.fn.expand '%:t' --[[@as string]]
  if not vim.endswith(bufname, '.txt.stderr') then
    return nil
  end
  return dir, tonumber(string.sub(bufname, 1, 4))
end

vim.api.nvim_create_user_command(
  'Bundle',
  string.format('!cargo xtask bundle | %s', clipboard),
  {}
)
vim.api.nvim_create_user_command('Run', '!cargo xtask test', {})
vim.api.nvim_create_user_command('Visualize', function(ctx)
  local id = tonumber(ctx.fargs[1])
  local in_file_name = make_file_path(nil, 'in', id)
  local out_file_name = make_file_path(nil, 'out', id)
  vim.cmd(string.format('!%s < %s', clipboard, in_file_name))
  vim.cmd 'sleep 1'
  vim.cmd(string.format('!%s < %s', clipboard, out_file_name))
end, { nargs = 1 })
vim.api.nvim_create_user_command('View', function(ctx)
  local id = tonumber(ctx.fargs[1])
  local idstr = string.format('%04d', id)
  vim.cmd 'tabnew'
  vim.cmd(string.format('edit testing/in/%s.txt', idstr))
  vim.cmd 'vsplit'
  vim.cmd(string.format('edit testing/out/%s.txt.stdout', idstr))
end, { nargs = 1 })
vim.api.nvim_create_user_command('Result', function(ctx)
  open_result(nil, tonumber(ctx.fargs[1]))
end, { nargs = 1 })
vim.api.nvim_create_user_command('ResultNext', function()
  local dir, id = parse_bufname()
  if id then
    open_result(dir, id + 1)
  end
end, {})
vim.api.nvim_create_user_command('ResultPrev', function()
  local dir, id = parse_bufname()
  if id then
    open_result(dir, id - 1)
  end
end, {})

vim.keymap.set({ 'n' }, '<C-S-N>', '<cmd>ResultNext<CR>')
vim.keymap.set({ 'n' }, '<C-S-P>', '<cmd>ResultPrev<CR>')
