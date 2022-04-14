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

static RUN_TEMPLATE_NAME: &str = "run.template";

fn template_file_name(template: &Template) -> String {
  match template {
    Template::Make => "Makefile.template",
    Template::CMake => "CMakeLists.Template",
    Template::Conan => "conanfile.Template",
  }.to_string()
}

#[derive(Debug)]
pub struct RunOptions {
  pub input: String,
  pub input_path: PathBuf,

  pub script_path: Option<PathBuf>,
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
  pub resource_directory: PathBuf,

  pub supress_run: bool,
  pub run_options: RunOptions,

  pub build_template: Template,
  pub template_file_path: PathBuf,
}

static AUTHOR: &str = "Virghileanu Teodor <@GaussianWonder>";
static GITIGNORE: &str = r###"
lex.yy.c
y.tab.c
y.tab.h
run.sh
build/*
{EXEC_NAME}
"###;

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
      .default_value("resources")
      .required(false))
    .arg(Arg::new("output")
      .short('o')
      .long("output")
      .value_name("OUPUT_DIRECTORY")
      .help("Path to the output folder of the generated files relative to the resource directory")
      .takes_value(true)
      .default_value("build")
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

fn assert_directory_exists(path: &PathBuf) {
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

pub fn get() -> Result<Config, Box<dyn std::error::Error>> {
  let args = get_raw();

  let resource_name: String = args.value_of("RESOURCE").unwrap().to_string();
  let yacc_file = format!("{}.y", resource_name);
  let lex_file = format!("{}.l", resource_name);

  let scan_directory: PathBuf = PathBuf::from(
    args.value_of("scan_directory").unwrap().to_string()
  );

  assert_directory_exists(&scan_directory);

  let resource_directory = scan_directory.join(&resource_name);

  let global_run_template = scan_directory.join(RUN_TEMPLATE_NAME);
  let local_run_template = resource_directory.join(RUN_TEMPLATE_NAME);
  let run_script_template = match (global_run_template.exists(), local_run_template.exists()) {
    (_, true) => Some(local_run_template),
    (true, false) => Some(global_run_template),
    _ => None,
  };

  let yacc_path = PathBuf::from(resource_directory.join(&yacc_file));
  let lex_path = PathBuf::from(resource_directory.join(&lex_file));

  match (yacc_path.exists(), lex_path.exists()) {
    (true, true) => {},
    _ => {
      panic!("Incomplete resource provided!");
    },
  }

  let output_directory: PathBuf = resource_directory.join(
    args.value_of("output").unwrap().to_string()
  );

  assert_directory_exists(&output_directory);

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
    let input_path = resource_directory.join(input);

    RunOptions {
      input: String::from(input),
      input_path,
      script_path: run_script_template,
    }
  } else {
    RunOptions {
      input: "input.txt".to_string(),
      input_path: resource_directory.join("input.txt"),
      script_path: run_script_template,
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

  let template_name = template_file_name(&build_template);
  let resource_template = resource_directory.join(&template_name);
  let template_file_path = if resource_template.exists() {
    resource_template.clone()
  } else {
    let global_template = PathBuf::from(&template_name);
    if global_template.exists() {
      global_template
    } else {
      panic!("No template found matching {} in resource directory or executable path", template_name);
    }
  };

  let mut gitignore_file = File::create(
    PathBuf::from(&resource_directory)
      .join(".gitignore")
  )?;
  gitignore_file.write(
    GITIGNORE
      .replace("{EXEC_NAME}", &resource_name)
      .as_bytes()
  )?;

  Ok(Config {
    resource_name,

    yacc_file,
    lex_file,

    yacc_path,
    lex_path,

    scan_directory,
    output_directory,
    resource_directory,

    supress_run,
    run_options,

    build_template,
    template_file_path,
  })
}