use std::process;

use clap::Parser;
use serde::Deserialize;
use source_map_mappings::{parse_mappings, Bias, Mappings};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "index.js.map")]
    source_map: String,

    line_no: u32,
    col_no: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourceMap {
    _version: usize,
    sources: Vec<String>,
    _sources_root: Option<String>,
    sources_content: Option<Vec<String>>,
    mappings: String,
    names: Vec<String>,
    _file: Option<String>,
}

// TODO: Print with nice syntax highlighting
fn main() {
    let args = Args::parse();

    let file_path = std::path::Path::new(&args.source_map);
    let full_path = match file_path.canonicalize() {
        Ok(r) => r,
        Err(e) => {
            println!("Could not open {}: {}", &args.source_map, e);
            process::exit(1)
        }
    };
    let os_file_path = full_path.to_str().unwrap();
    let file_contents = match std::fs::read(os_file_path) {
        Ok(r) => r,
        Err(e) => {
            println!("Could not open {}: {}", os_file_path, e);
            process::exit(2)
        }
    };
    let source_map: SourceMap = match serde_json::from_slice(&file_contents) {
        Ok(r) => r,
        Err(e) => {
            println!("Could not parse {}: {}", os_file_path, e);
            process::exit(3)
        }
    };

    let mappings: Mappings<()> = match parse_mappings(source_map.mappings.as_bytes()) {
        Ok(r) => r,
        Err(e) => {
            println!("Could not parse mappings: {:?}", e);
            process::exit(4)
        }
    };

    let original_location =
        match mappings.original_location_for(args.line_no - 1, args.col_no, Bias::default()) {
            Some(r) => r,
            None => {
                println!("Could not find original location");
                process::exit(5)
            }
        };
    let original = match &original_location.original {
        Some(r) => r,
        None => {
            println!("Could not find original location");
            process::exit(6)
        }
    };
    let source = original.source;
    let filename = source_map.sources[source as usize].clone();
    let source_content = match &source_map.sources_content {
        Some(r) => Some(&r[source as usize]),
        None => None,
    };
    let name = match original.name {
        Some(n) => format!("#{}", source_map.names[n as usize]),
        None => "".to_string(),
    };
    println!(
        "Source file: {}:{}:{}{}\n",
        filename,
        original.original_line + 1,
        original.original_column,
        name,
    );

    let excerpt_start_line = original.original_line.checked_sub(10).unwrap_or(0);
    let excerpt_end_line = original.original_line + 10;
    let excerpt = match source_content {
        Some(content) => Some(
            content
                .lines()
                .enumerate()
                .filter(|(i, _)| {
                    *i >= excerpt_start_line as usize && *i <= excerpt_end_line as usize
                })
                .map(|(i, l)| format!("{:4} {}", i + 1, l))
                .collect::<Vec<String>>()
                .join("\n"),
        ),
        None => None,
    };
    if let Some(content) = excerpt {
        println!("```\n{}\n```", &content)
    }
}
