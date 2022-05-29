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

pub const CURRENT_VERSION: u32 = 3;

#[derive(PartialEq)]
enum Modes {
    Version,
    Language,
    Branch,
    OS,
    Other,
}

pub enum Tokens {
    Language,
    Branch,
    OS,
    Other,

    Push,
    Push5,
    Push10,
    Push50,
    Push100,
    Sub,

    Print,
}

pub struct Identicode {
    lang_list: Vec<String>,
    branch_list: Vec<String>,
    os_list: Vec<String>,
    other_list: Vec<String>,

    mode: Modes,

    stack: u64,

    pub version: u32,
    pub languages: Vec<String>,
    pub branches: Vec<String>,
    pub oses: Vec<String>,
    pub others: Vec<String>,
}

impl Tokens {
    fn val(ch: char) -> Self {
        use self::Tokens::*;

        match ch {
            '$' => Language,
            '@' => Branch,
            '?' => OS,
            '&' => Other,

            '+' => Push,
            '*' => Push5,
            '/' => Push10,
            '%' => Push50,
            '=' => Push100,
            '-' => Sub,

            ';' => Print,
            _ => {
                println!("\x1b[1;31merror:\x1b[0m unknown character '{}'", ch);
                std::process::exit(1);
            }
        }
    }
}

impl Default for Identicode {
    fn default() -> Self {
        let langs = String::from_utf8_lossy(include_bytes!("data/langs.txt"))
            .lines()
            .map(str::to_string)
            .collect();
        let branches = String::from_utf8_lossy(include_bytes!("data/branches.txt"))
            .lines()
            .map(str::to_string)
            .collect();
        let oses = String::from_utf8_lossy(include_bytes!("data/oses.txt"))
            .lines()
            .map(str::to_string)
            .collect();
        let others = String::from_utf8_lossy(include_bytes!("data/others.txt"))
            .lines()
            .map(str::to_string)
            .collect();

        Identicode {
            version: 0,
            mode: Modes::Version,
            stack: 0,
            languages: vec![],
            branches: vec![],
            oses: vec![],
            others: vec![],
            lang_list: langs,
            branch_list: branches,
            other_list: others,
            os_list: oses,
        }
    }
}

impl Identicode {
    pub fn init(&mut self, file_data: String) {
        use Tokens::*;

        for ch in file_data.chars() {
            match Tokens::val(ch) {
                Language => {
                    self.mode = Modes::Language;
                }
                Branch => {
                    self.mode = Modes::Branch;
                }
                OS => {
                    self.mode = Modes::OS;
                }
                Other => {
                    self.mode = Modes::Other;
                }

                Push => {
                    self.stack += 1;
                }
                Push5 => {
                    self.stack += 5;
                }
                Push10 => {
                    self.stack += 10;
                }
                Push50 => {
                    self.stack += 50;
                }
                Push100 => {
                    self.stack += 100;
                }
                Sub => {
                    self.stack -= 1;
                }

                Print => {
                    fn push_item(vect: &mut Vec<String>, list: &mut [String], stack: u64) {
                        if !list[stack as usize].is_empty() {
                            vect.push(list[stack as usize].clone());
                            list[stack as usize] = "".to_string();
                        }
                    }

                    match self.mode {
                        Modes::Language => {
                            if self.stack < self.lang_list.len() as u64 {
                                push_item(&mut self.languages, &mut self.lang_list, self.stack);
                            }
                        }
                        Modes::Branch => {
                            if self.stack < self.branch_list.len() as u64 {
                                push_item(&mut self.branches, &mut self.branch_list, self.stack);
                            }
                        }
                        Modes::OS => {
                            if self.stack < self.lang_list.len() as u64 {
                                push_item(&mut self.oses, &mut self.os_list, self.stack);
                            }
                        }
                        Modes::Other => {
                            if self.stack < self.other_list.len() as u64 {
                                push_item(&mut self.others, &mut self.other_list, self.stack);
                            }
                        }
                        Modes::Version => {
                            self.version = self.stack as u32;
                        }
                    }

                    self.stack = 0;
                }
            }
        }
    }
}
