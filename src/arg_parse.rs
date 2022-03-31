use clap::{Arg, Command, ArgMatches};
use std::path::PathBuf;
use std::fs::{ File, create_dir_all };
use std::io::prelude::*;

#[derive(Debug)]
pub enum Template {
  Make,
  CMake,
  Conan,
}

#[derive(Debug)]
pub struct RunOptions {
  pub input: String,
  pub input_path: PathBuf,
}

#[derive(Debug)]
pub struct Config {
  pub resource_name: String,

  pub yacc_file: String,
  pub lex_file: String,

  pub yacc_path: PathBuf,
  pub lex_path: PathBuf,

  pub scan_directory: PathBuf,
  pub output_directory: PathBuf,

  pub supress_run: bool,
  pub run_options: RunOptions,

  pub build_template: Template,
}

static AUTHOR: &str = "Virghileanu Teodor <@GaussianWonder>";

fn get_raw() -> ArgMatches {
  Command::new("generator")
    .version("0.0.1")
    .author(AUTHOR)
    .about("Markdown generator")
    .arg(Arg::new("RESOURCE")
      .help("Resource name to look for")
      .required(true)
      .index(1))
    .arg(Arg::new("scan_directory")
      .short('i')
      .long("input")
      .value_name("SCAN_DIRECTORY")
      .help("Path to scan directory to start looking for a resource")
      .takes_value(true)
      .default_value("./resources")
      .required(false))
    .arg(Arg::new("output")
      .short('o')
      .long("output")
      .value_name("OUPUT_DIRECTORY")
      .help("Path to the output folder of the generated files")
      .takes_value(true)
      .default_value("./build")
      .required(false))
    .arg(Arg::new("build")
      .short('b')
      .long("build")
      .help("Mark the process as build only. This supresses the run subcommand")
      .required(false))
    .arg(Arg::new("make")
      .long("make")
      .help("Use the make template. This is the default")
      .required(false))
    .arg(Arg::new("cmake")
      .long("cmake")
      .help("Use the cmake template")
      .required(false))
    .arg(Arg::new("conan")
      .long("conan")
      .help("Use the conan template")
      .required(false))
    .subcommand(Command::new("run")
      .version("0.0.1")
      .author(AUTHOR)
      .about("Configure input test run behavior")
      .arg(Arg::new("file")
        .short('f')
        .long("file")
        .help("File to pipe the executable through")
        .takes_value(true)
        .default_value("input.txt")
        .required(false)))
    .get_matches()
}

fn assure_directory_exists(path: &PathBuf) {
  match create_dir_all(&path) {
    Ok(_) => {},
    Err(e) => {
      panic!(
        "Error handling the {}. Is it valid? Did you create it? Does it contain the given resource?\n\n{:?}",
        path.display(),
        e
      );
    }
  }
}

pub fn get() -> Config {
  let args = get_raw();

  let resource_name: String = args.value_of("RESOURCE").unwrap().to_string();
  let yacc_file = format!("{}.y", resource_name);
  let lex_file = format!("{}.l", resource_name);

  let scan_directory: PathBuf = PathBuf::from(
    args.value_of("scan_directory").unwrap().to_string()
  );

  assure_directory_exists(&scan_directory);

  let yacc_path = PathBuf::from(scan_directory.join(&yacc_file));
  let lex_path = PathBuf::from(scan_directory.join(&lex_file));

  match (yacc_path.exists(), lex_path.exists()) {
    (true, true) => {},
    (_, _) => {
      panic!("Incomplete resource provided!");
    },
  }

  let output_directory: PathBuf = scan_directory.join(
    args.value_of("output").unwrap().to_string()
  );

  assure_directory_exists(&output_directory);

  let supress_run: bool = args.is_present("build");

  let make = args.is_present("make");
  let cmake = args.is_present("cmake");
  let conan = args.is_present("conan");

  let build_template = match (make, cmake, conan) {
    (false, false,  false) => Template::Make,
    (true,  false,  false) => Template::Make,
    (false, true,   false) => Template::CMake,
    (false, false,  true)  => Template::Conan,
    (_,     _,      _)     => {
      panic!("Only one template can be used at a time!");
    },
  };

  let run_options = if let Some(run_args) = args.subcommand_matches("run") {
    let input = run_args.value_of("file").unwrap();
    let input_path = scan_directory.join(input);

    RunOptions {
      input: String::from(input),
      input_path,
    }
  } else {
    RunOptions {
      input: "input.txt".to_string(),
      input_path: scan_directory.join("input.txt"),
    }
  };

  if !run_options.input_path.exists() {
    println!("Creating input file: {}", run_options.input_path.display());
    let file = File::create(&run_options.input_path);
    match file {
      Err(e) => {
        panic!("Error creating input file: {}", e);
      },
      Ok(mut f) => {
        f.write_all(b"\n").ok();
      },
    }
  }

  Config {
    resource_name,

    yacc_file,
    lex_file,

    yacc_path,
    lex_path,

    scan_directory,
    output_directory,

    supress_run,
    run_options,

    build_template,
  }
}