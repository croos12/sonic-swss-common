use std::collections::HashSet;

struct Options {
    help: bool,
    unixsocket: bool,
    namespace: String,
}

fn parse_cli_arguments(args: Vec<String>) -> Result<Options, String> {
    let mut options = Options{help: false, unixsocket: false, namespace: "".to_string()};
    let mut ignore_arg: i32 = -1;

    for i in 1..args.len() {
        let arg = &args[i];
        if i as i32 == ignore_arg {
            continue;
        }
        match arg.as_str() {
            "--help" | "-h" => options.help = true,
            "--unixsocket" | "-s" => options.unixsocket = true,
            "--namespace" | "-n"=> {
                options.namespace = args[i+1].clone();
                ignore_arg = i as i32;
            }
            _ => return Err(format!("Unknown argument: {}", arg)),
        }
    }
    Ok(options)
}

pub fn sonic_db_cli(args: Vec<String>) -> Result<(), String> {
    parse_cli_arguments(args);
    Ok(())
}