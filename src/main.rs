mod arg_parse;
mod build;

fn main() {
	if let Ok(config) = arg_parse::get() {
		println!("{:#?}\n", config);

		let build_process = build::prepare_template(&config)
			.and_then(|_| build::create_run_script(&config))
			.and_then(|_| build::run_script(&config));

		if let Err(e) = build_process {
			println!("Error in the build process: {:?}", e);
		}
	} else {
		panic!("Couldn't get a valid config!");
	}
}
