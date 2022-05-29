// Copyright 2022 gulje 

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy
// of the License at

//   http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

fn print_info(vect: Vec<String>) {
    if vect.len() == 0 {
        println!("None");
    } else {
        for item in vect {
            println!("  * {}", item);
        }
    }
}

fn main() {
    use identicode::{Identicode};

    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("identicode - code that identifies you\n\
        Usage:\n\
        \t{} <code>", args.last().unwrap());

        std::process::exit(1)
    }

    let mut init = Identicode::default();
    init.init(std::env::args().last().unwrap());

    println!("\x1b[1;93mLanguages\x1b[0m:");
    print_info(init.languages);

    println!("\n\x1b[1;92mBranches\x1b[0m:");
    print_info(init.branches);

    println!("\n\x1b[1;94mOperating Systems\x1b[0m:");
    print_info(init.oses);

    println!("\n\x1b[1;90mOthers\x1b[0m:");
    print_info(init.others);
}
