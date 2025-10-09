-- main.lua — test script for ZephirEngine Lua integration

-- Verify sandbox_path injected by ZephirEngine
print("Sandbox Path:", sandbox_path)

-- Perform some CPU + I/O simulation
print("Performing a quick computation...")

local sum = 0
for i = 1, 100000 do
    sum = sum + i
end

print("Computation complete. Sum =", sum)

-- Write something into the sandbox (if permitted)
-- Demonstrate error handling
local ok, err = pcall(function()
    error("Intentional test error from Lua")
end)

if not ok then
    print("Caught error safely:", err)
end

print("Lua script execution complete ✅")
