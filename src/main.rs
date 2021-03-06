/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::io::stdin;
use std::io::Read;

#[macro_use]
extern crate clap;
use clap::App;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/256.rs"));

fn main ()
{
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let opt_action_decode = args.is_present("action_decode");

    if opt_action_decode
    {
        unimplemented!();
    }
    else
    {
        for byte in stdin().bytes()
        {
            match byte
            {
                Ok(x) => print!("{} ", WL_AUTOCOMPLETE[x as usize]),
                _ => break
            }
        }
    }
}
