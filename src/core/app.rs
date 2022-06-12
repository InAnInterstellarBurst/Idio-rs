// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::PathBuf;

use crate::logger;

#[derive(Default)]
pub struct ApplicationInfo
{
	pub name: &'static str,
	pub pref_path: PathBuf
}

pub trait Application
{
	fn init(&mut self, info: ApplicationInfo);
	fn tick(&self);
	fn deinit(&mut self);
}

pub fn run<T: Application>(name: &'static str, mut app: T)
{
	let mut datapath = match dirs::data_local_dir() {
		Some(d) => d,
		None => PathBuf::from("./")
	};

	let datasubdir: PathBuf = ["idio".to_string(), name.to_lowercase()].iter().collect();
	datapath.push(datasubdir);
	if fs::create_dir_all(&datapath).is_err() {
		println!("Preinit warning: Using ./ as prefpath");
		datapath = PathBuf::from("./");
	}

	let ai = ApplicationInfo {
		name: name,
		pref_path: datapath
	};

	logger::init(&ai);
	app.init(ai);
	for _ in 1..10000 {
		app.tick();
	}
	app.deinit();
}