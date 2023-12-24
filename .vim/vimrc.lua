local is_unix = vim.fn.has("unix") == 1
local is_win = vim.fn.has("win32") == 1
local is_wsl = is_unix and vim.fn.stridx(vim.fn.system("uname -a"), "WSL2") >= 0

local clipboard
if is_unix and vim.fn.executable("wl-copy") ~= 0 then
	clipboard = "wl-copy"
elseif is_unix and vim.fn.executable("xsel") ~= 0 then
	clipboard = "xsel -bo"
elseif is_win or is_wsl then
	clipboard = "clip.exe"
end

local function open_result(dir, id)
	dir = dir or "testing/out"
	if id < 0 then
		return
	end
	local bufname = vim.fn.expand("%:t")
	if not vim.endswith(bufname, ".txt.stderr") then
		vim.cmd("tabnew")
	end
	id = string.format("%04d", id)
	vim.cmd(string.format("edit %s/%s.txt.stderr", dir, id))
end

local function parse_bufname()
	local dir = vim.fn.expand("%:h")
	local bufname = vim.fn.expand("%:t")
	if not vim.endswith(bufname, ".txt.stderr") then
		return nil
	end
	return dir, tonumber(string.sub(bufname, 1, 4))
end

vim.api.nvim_create_user_command("Bundle", string.format("!cargo xtask bundle | %s", clipboard), {})
vim.api.nvim_create_user_command("Run", "!python testing/run_tests.py", {})
vim.api.nvim_create_user_command("Visualize", function(ctx)
	local id = string.format("%04d", tonumber(ctx.args[1]))
	local in_file_name = vim.fn.expand(string.format("testing/in/%s.txt", id))
	local out_file_name = vim.fn.expand(string.format("testing/out/%s.txt", id))
	vim.cmd(string.format("!%s < %s", clipboard, in_file_name))
	vim.cmd("sleep 1")
	vim.cmd(string.format("!%s < %s", clipboard, out_file_name))
end, { nargs = 1 })
vim.api.nvim_create_user_command("Result", function(ctx)
	open_result(nil, tonumber(ctx.args[1]))
end, { nargs = 1 })
vim.api.nvim_create_user_command("ResultNext", function()
	local dir, id = parse_bufname()
	if id then
		open_result(dir, id + 1)
	end
end, {})
vim.api.nvim_create_user_command("ResultPrev", function()
	local dir, id = parse_bufname()
	if id then
		open_result(dir, id - 1)
	end
end, {})

vim.keymap.set({ "n" }, "<C-S-N>", "<cmd>ResultNext<CR>")
vim.keymap.set({ "n" }, "<C-S-P>", "<cmd>ResultPrev<CR>")
