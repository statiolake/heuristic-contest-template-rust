local cmd = require("rc.lib.command")
local k = require("rc.lib.keybind")
local vimfn = require("rc.lib.vimfn")

local function open_result(dir, id)
	dir = dir or "testing/out"
	if id < 0 then
		return
	end
	local bufname = vim.fn.expand("%:t")
	if not string.ends_with(bufname, ".txt.stderr") then
		vim.cmd("tabnew")
	end
	id = string.format("%04d", id)
	vim.cmd(string.format("edit %s/%s.txt.stderr", dir, id))
end

local function parse_bufname()
	local dir = vim.fn.expand("%:h")
	local bufname = vim.fn.expand("%:t")
	if not string.ends_with(bufname, ".txt.stderr") then
		return nil
	end
	return dir, tonumber(string.sub(bufname, 1, 4))
end

cmd.add("Bundle", "!cargo xtask bundle | clip")
cmd.add("Run", "!python testing/run_tests.py")
cmd.add("Visualize", function(ctx)
	local id = string.format("%04d", tonumber(ctx.args[1]))
	local in_file_name = vimfn.expand(string.format("testing/in/%s.txt", id))
	local out_file_name = vimfn.expand(string.format("testing/out/%s.txt", id))
	vim.cmd(string.format("!clip < %s", in_file_name))
	vim.cmd("sleep 1")
	vim.cmd(string.format("!clip < %s", out_file_name))
end, { nargs = 1 })
cmd.add("Result", function(ctx)
	open_result(nil, tonumber(ctx.args[1]))
end, { nargs = 1 })
cmd.add("ResultNext", function()
	local dir, id = parse_bufname()
	if id then
		open_result(dir, id + 1)
	end
end)
cmd.add("ResultPrev", function()
	local dir, id = parse_bufname()
	if id then
		open_result(dir, id - 1)
	end
end)

k.nno("<C-S-N>", k.cmd("ResultNext"))
k.nno("<C-S-P>", k.cmd("ResultPrev"))
