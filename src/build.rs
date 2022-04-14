use crate::arg_parse::{ Config, Template };
use std::path::PathBuf;
use std::fs::{ File, read_to_string, OpenOptions };
use run_script::ScriptOptions;
use std::error::Error;
use std::io::{BufReader, BufRead, Write};

static MAKE_RUN: &str = r###"#!/bin/bash
make
cat {INPUT_NAME} | ./{EXEC_NAME} {INPUT_NAME}
"###;

fn run_script_content(config: &Config) -> String {
  match config.build_template {
      Template::Make => config.run_options.script_path
          .as_ref()
          .and_then(|path| read_to_string(path).ok())
          .or_else(|| Some(MAKE_RUN.to_string()))
          .expect("No script template to build from"),
      _ => "".to_string(),
  }
}

fn resolved_template_name(template: &Template) -> String {
  match template {
    Template::Make => "Makefile",
    Template::CMake => "CMakeLists.txt",
    Template::Conan => "conanfile.py",
  }.to_string()
}

pub fn prepare_template(config: &Config) -> Result<(), Box<dyn Error>> {
  let mut file = File::create(
    PathBuf::from(&config.resource_directory)
      .join(resolved_template_name(&config.build_template))
  )?;
  file.write(
    read_to_string(&config.template_file_path)?
      .replace("{LEX_FILE}", &config.lex_file)
      .replace("{YACC_FILE}", &config.yacc_file)
      .replace("{EXEC_NAME}", &config.resource_name)
      .as_bytes()
  )?;
  Ok(())
}

pub fn create_run_script(config: &Config) -> Result<(), Box<dyn Error>> {
  if config.supress_run {
    return Ok(());
  }

  let mut file = File::create(
    PathBuf::from(&config.resource_directory)
      .join("run.sh")
  )?;
  file.write(
    run_script_content(&config)
      .replace("{EXEC_NAME}", &config.resource_name)
      .replace("{INPUT_NAME}", &config.run_options.input)
      .as_bytes()
  )?;
  Ok(())
}

pub fn run_script(config: &Config) -> Result<(), Box<dyn Error>> {
  if config.supress_run {
    return Ok(());
  }

  let file = OpenOptions::new()
    .read(true)
    .open(
      PathBuf::from(&config.resource_directory)
        .join("run.sh")
    )?;
  
  let no_shabang = BufReader::new(file)
    .lines().skip(1)
    .map(|x| x.unwrap())
    .collect::<Vec<String>>().join("\n");

  let runnable_script = format!(
    "#!/bin/bash\ncd {}\n{}",
    &config.resource_directory.to_str().unwrap(),
    no_shabang,
  );

  let (code, output, error) = run_script::run_script!(
    &runnable_script[..],
    &vec![],
    &ScriptOptions::new()
  )?;

  println!("Exit code: {}\nSTDOUT:\n{}\nSTDERR:\n{}", code, output, error);
  Ok(())
}