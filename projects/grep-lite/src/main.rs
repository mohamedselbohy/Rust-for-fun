use grep_lite::{show_usage, stderr_out};
use std::{collections::HashMap, env, io};

#[derive(Clone, Copy)]
struct Opt {
    max_frequency: u8,
    argument_count: u8,
}

const OPTION_OP: [(&str, Opt); 1] = [(
    "-f",
    Opt {
        max_frequency: 1,
        argument_count: 1,
    },
)];
const PARAMETER_SIZE: u8 = 1;
fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    if (args).contains(&"-h".to_string()) {
        show_usage();
    }
    let mut options: HashMap<&str, Vec<String>> = HashMap::new();

    let mut option_size: HashMap<&str, u8> = HashMap::new();
    let mut parameters: Vec<String> = Vec::new();
    let mut nextind: i32 = -1;
    for ind in 0..args.len() {
        if nextind != -1 && nextind != ind as i32 {
            continue;
        }
        nextind = -1;
        let arg = args[ind].as_str();
        let opt = OPTION_OP.iter().find(|&&x| x.0 == arg);
        if arg.starts_with('-') && opt.is_none() {
            stderr_out(format!("Invalid option `{}`!", arg).as_str());
            show_usage();
        } else if !arg.starts_with('-') {
            if ind != 0 {
                if parameters.len() as u8 + 1 > PARAMETER_SIZE {
                    stderr_out(format!("invalid parameter: `{}`", arg).as_str());
                    show_usage();
                }
                parameters.push(arg.to_string());
            }
        } else {
            let option = opt.unwrap().1;
            option_size.insert(
                arg,
                match option_size.get(arg) {
                    Some(&u) => {
                        if u + 1 > option.max_frequency {
                            stderr_out(
                                format!(
                                    "You can use the option `{}` at most {} time{}!",
                                    arg,
                                    option.max_frequency,
                                    if option.max_frequency > 1 { 's' } else { '\0' }
                                )
                                .as_str(),
                            );
                            show_usage();
                            0
                        } else {
                            u + 1
                        }
                    }
                    None => 1,
                },
            );
            if ind as u8 + option.argument_count >= args.len() as u8 {
                stderr_out(
                    format!(
                        "The option `{}` needs {} argument{}!",
                        arg,
                        option.argument_count,
                        if option.argument_count > 1 { 's' } else { '\0' }
                    )
                    .as_str(),
                );
                show_usage();
            }
            let mut opt_args =
                args[ind + 1..(ind + option.argument_count as usize + 1 as usize)].to_vec();
            let opt_len = opt_args.len();
            for argument in &opt_args {
                if argument.starts_with('-') {
                    stderr_out(
                        format!(
                            "The option `{}` needs {} argument{}!",
                            arg,
                            option.argument_count,
                            if option.argument_count > 1 { 's' } else { '\0' }
                        )
                        .as_str(),
                    );
                    show_usage();
                }
            }
            nextind = 1 + ind as i32 + opt_len as i32;
            match options.get_mut(arg) {
                Some(v) => {
                    v.append(&mut opt_args);
                }
                None => {
                    options.insert(arg, opt_args);
                }
            }
        }
    }

    if parameters.len() < PARAMETER_SIZE as usize {
        stderr_out(
            format!(
                "You have to enter at least {} matching parameter{}!",
                PARAMETER_SIZE,
                if PARAMETER_SIZE > 1 { 's' } else { '\0' }
            )
            .as_str(),
        );
        show_usage();
    }

    if options.get("-f").is_none() {
        let input = {
            let mut _in = String::new();
            io::stdin().read_line(&mut _in).expect("Couldn't read text");
            _in
        };
        grep_lite::print::print(grep_lite::stdio::search(
            &parameters[0],
            input.as_str(),
            &options,
        ));
    } else {
        // todo!("choose mode between -f or reading from stdin. Then searching through input for pattern.");
    }
}
