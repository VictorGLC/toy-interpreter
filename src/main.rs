use std::collections::HashMap;

fn analyze(program: &str) -> (HashMap<String, usize>, HashMap<String, usize>) {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut function_table: HashMap<String, usize> = HashMap::new();
    let mut memory_address = 0;

    for (line_number, line) in program.lines().enumerate() {
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        match tokens.as_slice() {
            ["var", name] => {
                if symbol_table.contains_key(*name) {
                    eprintln!("variable redefined: {}", name);
                } else {
                    symbol_table.insert(name.to_string(), memory_address);
                    memory_address += 1;
                }
            }
            [name, "=", _number] => {
                if !symbol_table.contains_key(*name) {
                    eprintln!("variable unknown: {}", name);
                }
            }
            ["func", name, "{"] => {
                let func_name = format!("{}", name);
                if function_table.contains_key(&func_name) {
                    eprintln!("function redefined: {}", name);
                } else {
                    function_table.insert(func_name, line_number);
                }
            }
            ["}"] => {}
            [name] if name.ends_with("()") => {
                if !function_table.contains_key(*name) {
                    eprintln!("function unknown: {}", name);
                }
            }
            _ => {
                eprintln!("unmatched: {}", line);
            }
        }
    }

    (symbol_table, function_table)
}

fn execute(program: &str, symbol_table: &HashMap<String, usize>, function_table: &HashMap<String, usize>) {
    let lines: Vec<&str> = program.lines().map(str::trim).collect();
    let mut memory = vec![0; symbol_table.len()];
    let mut call_stack: Vec<usize> = Vec::new();
    let mut pc = 0;

    while pc < lines.len() {
        let tokens: Vec<&str> = lines[pc].split_whitespace().collect();
        //println!("{:?}", tokens);
        if tokens.is_empty() {
            pc += 1;
            continue;
        }

        match tokens.as_slice() {
            ["var", _name] => {}
            [name, "=", number] => {
                if let Some(&address) = symbol_table.get(*name) {
                    if let Ok(value) = number.parse::<i32>() {
                        memory[address] = value;
                        println!("{} at address {} receives {}", name, address, value);
                    }
                }
            }
            ["func", _name, "{"] => {
                while pc < lines.len() && lines[pc] != "}" {
                    pc += 1;
                }
            }
            [name] if name.ends_with("()") => {
                if let Some(&target_line) = function_table.get(*name) {
                    println!("{} called in line {}", name, pc);
                    call_stack.push(pc);
                    pc = target_line+1;
                    continue;
                }
            }
            ["}"] => {
                if let Some(return_line) = call_stack.pop() {
                    println!("return to line {}", return_line);
                    pc = return_line;
                }
            }
            _ => {}
        }

        pc += 1;
    }

    println!("execution ended\n");
    println!("memory: {:?}", memory);
    println!("call_stack: {:?}", call_stack);
}

fn main() {
    let program = r#"var a
        func f() {
            a = 5
            var b
            b = 6
        }
        func g() {
            var c
            c = 7
            f()
        }
        g()
    "#;

    let (symbol_table, function_table) = analyze(program);
    println!("symbol_table: {:?}", symbol_table);
    println!("function_table: {:?}", function_table);
    execute(program, &symbol_table, &function_table);
}