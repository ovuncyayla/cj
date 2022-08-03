local status_ok, dap = pcall(require, "dap")
if not status_ok then
  vim.notify("Unable to load dap")
  return
end


dap.configurations = {
  rust = {
    {
      name = "Launch file siki",
      type = "cppdbg",
      request = "launch",
      program = function()
        -- local function get_project(table)
        --   for index, value in pairs(table) do
        --     if value == "src" then
        --       return table[index - 1]
        --     end
        --   end
        -- end
        --
        -- local pname = get_project(require("user.custom.utils").split(vim.fn.expand "%", "/"))
        -- return vim.fn.expand "%:h" .. "/../target/debug/" .. pname
        return vim.fn.getcwd() .. "/target/debug/cj"
        -- return vim.fn.input("Path to executable: ", vim.fn.getcwd() .. "/", "file")
      end,
      cwd = "${workspaceFolder}",
      args = function ()
        return {"-p", "test.json"}
      end ,
      stopOnEntry = true,
      runInTerminal = true,
    MIMode= "gdb",
    miDebuggerPath= "/home/us3r/.cargo/bin/rust-gdb"
    },
 {
            name = "LLDB",
            type = "lldb",
            request = "launch",
            program = function()
                return vim.fn.input('Path to executable: ',
                                    vim.fn.getcwd() .. '/', 'file')
            end,
            cwd = '${workspaceFolder}',
            stopOnEntry = false,
      args = function ()
        return {"-p", "test.json"}
      end ,

            -- if you change `runInTerminal` to true, you might need to change the yama/ptrace_scope setting:
            --
            --    echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
            --
            -- Otherwise you might get the following error:
            --
            --    Error on launch: Failed to attach to the target process
            --
            -- But you should be aware of the implications:
            -- https://www.kernel.org/doc/html/latest/admin-guide/LSM/Yama.html
            runInTerminal = true,
    MIMode= "gdb",
    miDebuggerPath= "/home/us3r/.cargo/bin/rust-gdb",
setupCommands = {  
  { 
     text = '-enable-pretty-printing',
     description =  'enable pretty printing',
     ignoreFailures = false 
  },
        },
},
  },
}
