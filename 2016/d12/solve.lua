local function get_val(register, v)
    return tonumber(v) or register[v]
end


INSTRUCTIONS = {
    cpy = function(register, v, into)
        register[into] = get_val(register, v)
        return 1
    end,
    
    inc = function(register, v)
        register[v]  = register[v] + 1
        return 1
    end,
    
    dec = function(register, v)
        register[v]  = register[v] - 1
        return 1
    end,
    
    jnz = function(register, v, delta)
        if get_val(register, v) ~= 0 then
            return get_val(register, delta)
        end
        return 1
    end,
}


local unpack = unpack or table.unpack  -- luajit compat
local function execute(program, register)
    local ip = 1

    while ip <= #program do
        instruction = program[ip]
        ip = ip + instruction.exec(register, unpack(instruction.args))
    end
    return register
end


local function read_program()
    local program = {}
    for line in io.lines('input.txt') do
        inst_name, arg1, arg2 = line:match('(%l+) (%w+) ?(-?%w*)')
        program[#program + 1] = {exec = INSTRUCTIONS[inst_name], args = {arg1, arg2}}
    end
    return program
end


function part1()
    local register = {a = 0, b = 0, c = 0, d = 0}
    print(execute(read_program(), register).a)
end


function part2()
    local register = {a = 0, b = 0, c = 1, d = 0}
    print(execute(read_program(), register).a)
end



if arg[1] == '1' then
    part1()
else
    part2()
end
