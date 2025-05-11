use std::collections::HashMap;

fn main() {
    let program = 
    r#"
        var a
        func f() {
            a = 5
            var b
            b = 6
        }
        func g() {
            var c
            c = 7
        }
        f()
        g()
    "#;

    let (global_symbol_table, local_symbol_table, function_table) = analyze(program);
    println!("global_symbol_table: {:?}", global_symbol_table);
    println!("local_symbol_table: {:?}",local_symbol_table);
    println!("function_table: {:?}\n", function_table);

    let (memory, call_stack, activation_frames) = execute(program, &global_symbol_table, &function_table);
    println!("memory: {:?}", memory);
    println!("call_stack: {:?}", call_stack);
    println!("activation_frames: {:?}", activation_frames);
}

fn analyze(program: &str) -> (HashMap<String, usize>, Vec<String>,HashMap<String, usize>) {
    let mut global_symbol_table: HashMap<String, usize> = HashMap::new();
    let mut function_table: HashMap<String, usize> = HashMap::new();
    let mut memory_address: usize = 0;
    let mut is_in_function: bool = false;
    let mut local_symbol_table: Vec<String> = Vec::new();

    for (line_number, line) in program.trim().lines().enumerate() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();
   
        match line.as_slice() {
            ["var", name] => {
                if is_in_function {
                    if local_symbol_table.contains(&name.to_string()) || global_symbol_table.contains_key(*name) {
                        println!("variable redefined: {}", name);
                    } else {
                        local_symbol_table.push(name.to_string());
                    }
                } else {
                    if global_symbol_table.contains_key(*name) {
                        println!("variable redefined: {}", name);
                    } else {
                        global_symbol_table.insert(name.to_string(), memory_address);
                        memory_address += 1;
                    }
                }
            }
            [name, "=", _number] => {
                if is_in_function {
                    if !local_symbol_table.contains(&name.to_string()) && !global_symbol_table.contains_key(*name) {
                        eprintln!("variable unknown: {}", name);
                    }
                } else { 
                    if !global_symbol_table.contains_key(*name) {
                        eprintln!("variable unknown: {}", name);
                    }
                }
            }
            ["func", name, "{"] => {
                let func_name: String = format!("{}", name);
                if function_table.contains_key(&func_name) {
                    println!("function redefined: {}", name);
                } else {
                    function_table.insert(func_name, line_number);
                }
                is_in_function = true;
            }
            ["}"] => {
                if !local_symbol_table.is_empty() {
                    println!("clearing local_symbol_table: {:?}", local_symbol_table);
                    local_symbol_table.clear();
                }
                is_in_function = false;
            }
            [name] if name.ends_with("()") => {
                if !function_table.contains_key(*name) {
                    eprintln!("function unknown: {}", name);
                }
            }
            _ => {
                eprintln!("unmatched: {:?}", line);
            }
        }
    }
    println!("analysis ended\n");
    (global_symbol_table, local_symbol_table, function_table)
}

fn execute(program: &str, global_symbol_table: &HashMap<String, usize>, function_table: &HashMap<String, usize>) -> (Vec<i32>, Vec<usize>, Vec<HashMap<String, usize>>) {
    let lines: Vec<&str> = program.trim().lines().collect();
    let mut pc: usize = 0; // Program counter
    let mut memory: Vec<i32> = vec![0; global_symbol_table.len()];
    let mut call_stack: Vec<usize> = Vec::new();
    let mut activation_frames: Vec<HashMap<String, usize>> = Vec::new();
    let mut memory_address: usize = memory.len();

    let mut is_in_function: bool = false;

    while pc < lines.len() {
        let line: Vec<&str> = lines[pc].split_whitespace().collect();
        match line.as_slice() {
            ["var", name] => {
                if is_in_function && !global_symbol_table.contains_key(*name) {
                    memory.push(0);
                    activation_frames.last_mut().unwrap().insert(name.to_string(), memory_address);
                    println!("created local {} with address {}", name, memory_address);
                    memory_address += 1;
                }
            }
            [name, "=", number] => {
                let value = number.parse::<i32>().unwrap();
                if let Some(&address) = global_symbol_table.get(*name) {
                    memory[address] = value;
                    println!("{} at address {} receives {}", name, address, value);
                } else if let Some(address) = activation_frames.last().and_then(|frame| frame.get(*name)) {
                    memory[*address] = value;
                    println!("{} at address {} receives {}", name, address, value);
                }
            }
            ["func", _name, "{"] => {
                while lines[pc].trim() != "}" {
                    pc += 1;
                }
            }
            [name] if name.ends_with("()") => {
                is_in_function = true;
                if let Some(&func_line) = function_table.get(*name) {
                    println!("{} called in line {}", name, pc);
                    call_stack.push(pc);
                    activation_frames.push(HashMap::new());
                    pc = func_line;
                } else {
                    eprintln!("invalid function: {}", name);
                }
            }
            ["}"] => {
                is_in_function = false;
                println!("memory before removal of local variables: {:?}", memory);

                if let Some(frame) = activation_frames.pop() {
                    println!("deleting last activation frame: {:?}", frame);

                    memory.truncate(memory.len() - frame.len());
                    memory_address = memory_address - frame.len();

                }

                if let Some(return_line) = call_stack.pop() {
                    println!("return to line {}", return_line);
                    pc = return_line;
                }
            }
            _ => {
                eprintln!("unmatched: {:?}", line);
            }
        }
        pc += 1;
    }
    println!("execution ended\n");
    (memory, call_stack, activation_frames)
}