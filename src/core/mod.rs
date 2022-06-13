// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod app;
pub use app::*;

pub mod logger;
pub use logger::LogLevel;

pub mod window;
pub use window::*;

#[allow(unused_macros)]
macro_rules! log_engine
{
	($l:expr, $($t:tt)*) => {
		crate::logger::log("Idio", $l, format!($($t)*));
	}
}

#[allow(unused_imports)]
pub(crate) use log_engine;

#[macro_export]
macro_rules! log_game
{
	($l:expr, $($t:tt)*) => {
		idio::logger::log("Game", $l, format!($($t)*));
	}
}
