mod indentprint;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use walkdir::WalkDir;

const EXCLUDED_EXTENSIONS: &[&str] = &["jpg", "png", "exe", "gif", "pdb", "rmeta", "rlib", "bin"];
const HELP_LIST: &[[&str; 2]] = &[
    [".[ext]", "Include / exclude file extension"],
    ["--help | -h", "Show usage of loccer"],
    ["--exclude | -e", "Exclude given file extensions"],
    [
        "--minimum | -m",
        "Shows only total count of files and lines",
    ],
    ["--files | -f", "Shows every file count of lines"],
];

#[derive(Debug)]
struct Total {
    files: i32,
    lines: i32,
}

enum PrintStyle {
    Minimum,
    Normal,
    Files,
}

pub struct Config {
    dir_path: String,
    selected_file_extensions: Vec<String>,
    exclude: bool,
    print_style: PrintStyle,
}

impl Config {
    pub fn init(args: &[String]) -> Config {
        let execution_path = env::current_dir().unwrap();

        let mut dir_path = execution_path.into_os_string().into_string().unwrap();
        let mut selected_file_extensions: Vec<String> = Vec::new();
        let mut exclude = false;
        let mut print_style = PrintStyle::Normal;

        for arg in &args[1..] {
            if arg.contains('/') || arg.contains('\\') {
                dir_path = arg.to_string();
            } else if let Some(ext) = arg.strip_prefix('.') {
                selected_file_extensions.push(ext.to_string())
            } else if arg == "--exclude" || arg == "-e" {
                exclude = true;
            } else if arg == "--minimum" || arg == "-m" {
                print_style = PrintStyle::Minimum;
            } else if arg == "--files" || arg == "-f" {
                print_style = PrintStyle::Files;
            } else if arg == "help" || arg == "--help" || arg == "-h" {
                print_help();
                std::process::exit(0);
            } else {
                println!("Invalid parameter {}, try loccer --help for help", arg);
                std::process::exit(0);
            }
        }

        Config {
            dir_path,
            selected_file_extensions,
            exclude,
            print_style,
        }
    }
}

fn print_help() {
    println!("Usage: loccer [optional_arguments]");
    println!("Example: loccer ./ -e --files .js .c .rs");
    println!("Optional arguments:");

    for item in HELP_LIST {
        indentprint::print(item[0], "", 2, indentprint::Align::Left);
        indentprint::println(item[1], item[0], 20, indentprint::Align::Left);
    }
}

fn print_lines(count: usize) {
    let lines = "-".repeat(count);
    println!("{}", lines)
}

pub fn run(config: Config) {
    let mut code_total: HashMap<String, Total> = HashMap::new();
    let mut total: Total = Total { files: 0, lines: 0 };

    if matches!(config.print_style, PrintStyle::Files) {
        print_lines(74);
        println!(" File                                                               Lines ");
        print_lines(74);
    }

    for path in WalkDir::new(&config.dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = path.path();

        let mut file_extension = String::from("");

        if let Some(extension) = file_path.extension() {
            file_extension = extension.to_str().unwrap().to_string()
        }

        if (!config.exclude
            && !config.selected_file_extensions.contains(&file_extension)
            && !config.selected_file_extensions.is_empty())
            || (config.exclude
                && config.selected_file_extensions.contains(&file_extension)
                && !config.selected_file_extensions.is_empty())
            || EXCLUDED_EXTENSIONS.contains(&&*file_extension)
        {
            continue;
        }

        if let Some(t) = code_total.get(&*file_extension) {
            code_total.insert(
                file_extension.to_owned(),
                Total {
                    files: t.files + 1,
                    lines: t.lines,
                },
            );
        } else {
            code_total.insert(file_extension.to_owned(), Total { files: 1, lines: 0 });
        }
        total.files += 1;

        let file = fs::File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut file_lines = 0;

        for _ in reader.lines() {
            if let Some(t) = code_total.get(&*file_extension) {
                code_total.insert(
                    file_extension.to_owned(),
                    Total {
                        files: t.files,
                        lines: t.lines + 1,
                    },
                );
            } else {
                code_total.insert(file_extension.to_owned(), Total { files: 0, lines: 1 });
            }
            file_lines += 1;
        }

        if matches!(config.print_style, PrintStyle::Files) {
            let relative_path =
                file_path.display().to_string()[config.dir_path.to_string().len()..].to_string();
            print!(" .{:}", relative_path);
            indentprint::println(
                &file_lines.to_string(),
                &relative_path,
                70,
                indentprint::Align::Right,
            );
        }
        total.lines += file_lines;
    }

    print_lines(32);
    println!(" Language      Files      Lines ");
    print_lines(32);

    if !matches!(config.print_style, PrintStyle::Minimum) {
        for (k, v) in code_total.into_iter() {
            print!(" .{}", k);
            indentprint::print(&v.files.to_string(), &k, 17, indentprint::Align::Right);
            indentprint::println(&v.lines.to_string(), "", 10, indentprint::Align::Right);
        }

        print_lines(32);
    }
    print!(" Total");
    indentprint::print(&total.files.to_string(), "", 13, indentprint::Align::Right);
    indentprint::println(&total.lines.to_string(), "", 10, indentprint::Align::Right);
    print_lines(32);
}
