use std::fs::File;
use std::io::{Error, BufReader, BufRead};

#[derive(Debug)]
pub struct Flags { n: bool, l: bool, i: bool, v: bool, x: bool }

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            n: flags.contains(&"-n"),
            l: flags.contains(&"-l"),
            i: flags.contains(&"-i"),
            v: flags.contains(&"-v"),
            x: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let p = if flags.i { pattern.to_lowercase() } else { pattern.to_owned() };
    let mut res = vec![];
    for file in files {
        let f = File::open(file)?;
        let prefix = if files.len() > 1 { format!("{}:", file) } else { "".to_owned() };
        for (i, line) in BufReader::new(f).lines().enumerate() {
            let line = line?;
            let l = if flags.i { line.to_lowercase() } else { line.to_string() };
            let found = if flags.x { l == p } else { l.contains(&p) };
            if if flags.v { !found } else { found } {
                if flags.l {
                    res.push(file.to_string());
                    break;
                } else {
                    let prefix2 = if flags.n { format!("{}:", i + 1) } else { "".to_owned() };
                    res.push(format!("{}{}{}", prefix, prefix2, line));
                }
            }
        }
    }
    Ok(res)
}
