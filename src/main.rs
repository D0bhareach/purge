use lazy_static::lazy_static;
use regex::{self, Regex};
use std::{
    borrow::Cow,
    default::Default,
    env,
    fs::{self, OpenOptions},
    io::{Error as IOError, Write},
};

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("transparent")]
    RegErr(#[from] regex::Error),
    #[error("transparent")]
    IOErr(#[from] IOError),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Default)]
struct State<'a> {
    multiline: Vec<(MultilineType, usize, usize)>,
    lines: Vec<Line<'a>>,
}

#[derive(Debug)]
struct Line<'a> {
    ignore: bool,
    data: Cow<'a, str>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MultilineType {
    Fix,
    Purge,
    Todo,
}

lazy_static! {
    // line start with //
    static ref REG_COMMENT: Regex = Regex::new(r"^\s*//\s*").unwrap();
    // line has /* TODO or FIX
    static ref REG_MULTILINE_TODO: Regex = Regex::new(r"\s*/\*\s*TODO").unwrap();
    static ref REG_MULTILINE_FIX: Regex = Regex::new(r"\s*/\*\s*FIX").unwrap();
    // line starts with // PURGE
    static ref REG_START_PURGE: Regex = Regex::new(r"^\s*//\s*PURGE").unwrap();
    // line ends with // ENDPURGE
    static ref REG_END_PURGE: Regex = Regex::new(r"^\s*//\s*ENDPURGE").unwrap();
    // end of multiline comment
    static ref REG_END_MULTILINE: Regex = Regex::new(r"^\s*\*/").unwrap();
}

impl<'a> State<'a> {
    fn remove_inline(&mut self, line: &str) {
        let mut line = line.to_owned();
        while let Some(f1) = line.find("/*!!") {
            if let Some(f2) = line.find("*/") {
                let rep = &line[f1..=(f2 + 1)];
                line = line.replace(rep, "");
            }
        }
        self.lines.push(Line {
            ignore: false,
            data: Cow::Owned(line),
        });
    }
    fn multiline_add_start(&mut self, t: MultilineType, i: usize) -> Result<(), AppError> {
        self.multiline.push((t, i, 0));
        Ok(())
    }

    fn multiline_add_end(&mut self, t: &MultilineType, i: usize) -> Result<(), AppError> {
        // last one must be the one to close
        if let Some(mut e) = self.multiline.pop() {
            if t != &e.0 {
                return Err(AppError::Other(
                    "Not properly encapsulated multilines comments".to_owned(),
                ));
            }
            if e.2.eq(&0) {
                e.2 = i;
                if self.multiline.is_empty() {
                    self.multiline.push(e)
                } else {
                    self.multiline.insert(0, e)
                };
            } else if e.2.gt(&0) {
                // looks like we found comment without removal marker
                if self.multiline.is_empty() {
                    self.multiline.push(e)
                } else {
                    self.multiline.insert(0, e)
                };
            } else {
                return Err(AppError::Other("Last poped multiline not zero.".to_owned()));
            }
        } else {
            return Err(AppError::Other(
                "Error poping last element from multiline.".to_owned(),
            ));
        }

        Ok(())
    }

    fn validate_multiline(&mut self) -> Result<(), AppError> {
        let mut previous = 0usize;
        self.multiline.reverse();
        for pair in &self.multiline {
            let (_, x, y) = pair;
            if x < &previous {
                return Err(AppError::Other(
                    "malformed multiline pair, x < previous".to_owned(),
                ));
            } else if x > y {
                return Err(AppError::Other(
                    "malformed multiline pair, x > y".to_owned(),
                ));
            }
            previous = *y;
        }
        Ok(())
    }
    /// This function breacks rule of single responsibility. It take mutable parameters from outer scope
    /// and change them. It reads file as a string walks file line by line and eiter ignre lines if they
    /// match criteria, or add the to mutable vector.  
    fn first_cycle(&mut self, main_string: &'a str) -> Result<(), AppError> {
        let ignore = vec!["// FIX", "// TODO:"];
        let mut process_next = false;
        // I must check that type is the same for comment otherwise inpropor encapsulation is possible.
        let mut last_multiline_type = MultilineType::Fix;
        'main_loop: for (index, line) in main_string.lines().enumerate() {
            if REG_COMMENT.is_match(line) {
                if process_next {
                    self.lines.push(Line {
                        ignore: true,
                        data: Cow::Borrowed(line),
                    });
                    continue 'main_loop;
                } else {
                    for ign in ignore.iter() {
                        if line.contains(ign) {
                            process_next = true;
                            self.lines.push(Line {
                                ignore: true,
                                data: Cow::Borrowed(line),
                            });
                            continue 'main_loop;
                        }
                    }
                }
            } else {
                process_next = false;
            }
            // set flags for future processing.
            if REG_MULTILINE_TODO.is_match(line) {
                self.multiline_add_start(MultilineType::Todo, index)?;
                last_multiline_type = MultilineType::Todo;
            }
            if REG_MULTILINE_FIX.is_match(line) {
                self.multiline_add_start(MultilineType::Fix, index)?;
                last_multiline_type = MultilineType::Fix;
            }
            if REG_START_PURGE.is_match(line) {
                self.multiline_add_start(MultilineType::Purge, index)?;
                last_multiline_type = MultilineType::Purge;
            }
            if REG_END_PURGE.is_match(line) {
                self.multiline_add_end(&last_multiline_type, index)?;
            }

            if REG_END_MULTILINE.is_match(line) {
                self.multiline_add_end(&last_multiline_type, index)?;
            }
            // remove inline delete comments
            if line.contains("/*!!") {
                self.remove_inline(line);
                continue;
            }

            self.lines.push(Line {
                ignore: false,
                data: Cow::Borrowed(line),
            });
        }
        Ok(())
    }
} // impl state

// TODO: create function to remove all comments, except doc comments, regardles of their marking.
fn main() -> Result<(), AppError> {
    // get file name
    let mut a = env::args();
    if a.len() < 2 {
        return Err(AppError::Other(
            "Must provide name of file to work.".to_owned(),
        ));
    }
    // checked it already
    let fname = a.nth(1).unwrap();
    let mut state = State {
        lines: Vec::with_capacity(250),
        ..Default::default()
    };

    // average code is about 250 lines of code and less.
    let main_string = fs::read_to_string(&fname)?;

    state.first_cycle(&main_string)?;
    state.validate_multiline()?;
    for p in state.multiline {
        let (_, start, end) = p;
        let sl = &mut state.lines[start..=end];
        for l in sl.iter_mut() {
            l.ignore = true;
        }
    }
    state.lines.retain(|l| !l.ignore);

    let mut file = OpenOptions::new().write(true).truncate(true).open(fname)?;
    for l in state.lines.into_iter() {
        writeln!(file, "{}", l.data)?;
    }

    Ok(())
}
