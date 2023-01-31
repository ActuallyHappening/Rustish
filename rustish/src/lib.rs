use std::{path::Path, fs::File, io::Write};

pub trait JS {
		fn js_pls(&self) -> String;
}

impl JS for String {
		fn js_pls(&self) -> String {
				format!(r#"console.log(`(Auto Rust:String -> JS) {}`);"#, self)
		}
}

pub fn output_js<T: JS>(t: T, path: &Path) {
		let js = t.js_pls();
		println!("Creating file at path: {:?}", path);
		let mut file = File::create(path).expect("Unable to create file at path provided");
		file.write_all(js.as_bytes()).expect("Unable to write to file");
}