// Copyright (c) 2022 Connor Mellon
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Default)]
struct Test
{
    info: idio::ApplicationInfo
}

impl idio::Application for Test
{
    fn init(&mut self, info: idio::ApplicationInfo)
    {
        self.info = info;
        idio::log_game!(idio::LogLevel::Error, "{}", 4);
    }

    fn tick(&self)
    {
    }

    fn deinit(&mut self)
    {
    }
}

fn main()
{
    idio::run("TestApp", Test::default());
}
