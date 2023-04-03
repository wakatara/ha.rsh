// extern crate chrono;
// extern crate clap;
// extern crate dirs;

// use chrono::prelude::*;
use std::{fs::{File, OpenOptions}, 
          io::{BufRead, BufReader, Write}, 
          path::PathBuf,
         //  collections::HashMap
          };
use clap::{Parser, Subcommand};
pub(crate) use chrono::NaiveDate;

#[derive(Parser)]
#[command(author, version, about = "harsh - a minamalist CLI habit tracking app", long_about=None)]
struct Cli {

   name: Option<String>,

    //#[arg(short, long, value_name = "FILE")]
    //habits_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ask {

    },
    Log {
        stats: bool, 
    },
    Todo {

    },
}

#[derive(PartialEq)]
enum DayStatus {
    Unknown,
    NotDone,
    Done,
    Satisfied,
    Skipped,
    Skipified,
    Warning,
}

type Day = i64;

struct DayHabit {
    date: NaiveDate,
    habit: Habit,
    frequency: i64,
}

#[derive(Debug)]
struct Habit {
    heading: String,
    habit:   String,
    frequency: i64,
}

struct Harsh {
    habits_file: PathBuf,
    log_file: PathBuf,
    habits: Vec<Habit>,
    // entries: HashMap<DayHabit, DayStatus>,
}


fn main() {
    let cli = Cli::parse();
    let harsh = Harsh::new();
    harsh.load_habits();
    // harsh.load_entries();

    match &cli.command {
        Some(Commands::Ask {}) => {
            harsh.ask_habits();
        },
        Some(Commands::Log {..}) => {
            // harsh.build_spark();
            // harsh.build_consistency();
        },
        Some(Commands::Todo {}) => {
            // harsh.todos();
        },
        None => {}
    }
}

impl Harsh {

    fn new() -> Self {
        let harsh = Self::check_setup();
        harsh
    } 

    fn load_habits(&self) -> Vec<Habit> {
        let f = File::open(&self.habits_file).unwrap();
        let habits_file = BufReader::new(&f);

        let mut habits = vec![];
        let mut heading: Box<String> = Box::new("".to_string());
        for line in habits_file.lines() {
            let l = line.unwrap();
            if l.chars().count() > 0 {
                let first_char = l.chars().nth(0).unwrap(); 
                if first_char == '!' {
                    let mut split = l.split("! ");
                    *heading = String::from(split.nth(1).unwrap());
                } else if !(first_char == '#' || first_char == '\n') {
                    let splits: _ = l.split(": ");
                    let result:Vec<&str> = splits.collect();
                    habits.push(Habit {
                        heading: heading.to_string(), 
                        habit: result[0].to_string(),
                        frequency: result[1].parse().unwrap(),
                })}
        }}
        println!("{:#?}", habits);
        habits
    }

    fn ask_habits(&self) {
        
    }

    fn check_setup() -> Harsh {
        let mut harsh_dir = dirs::home_dir().unwrap();
        harsh_dir.push(".config/harsh");
        if !harsh_dir.is_dir() {
            println!("Welcome to harsh!\n");
            std::fs::create_dir(&harsh_dir).unwrap();
        }

        let mut habits_file = harsh_dir.clone();
        habits_file.push("habits");
        if !habits_file.is_file() {
            File::create(&habits_file).unwrap();
            
            let file = OpenOptions::new().append(true).open(&habits_file).unwrap();
            write!(
                &file,
                "# The numbers specifies how often you want to do a habit:\n"
            );
            write!(
                &file,
                "# 1 means daily, 7 means weekly, 0 means you're just tracking the habit. Some examples:\n"
            );
            write!(
                &file,
                "\n# 1 Meditated\n# 7 Cleaned the apartment\n# 0 Had a headache\n# 1 Used habitctl\n"
            );
            println!(
                "Created {}. This file will list your currently tracked habits.",
                habits_file.to_str().unwrap()
            
            );
        }

        let mut log_file = harsh_dir.clone();
        log_file.push("log");
        if !log_file.is_file() {
            std::fs::File::create(&log_file).unwrap();

            println!(
                "Created {}. This file will log your habits.\n",
                log_file.to_str().unwrap()
            );
        }

        Harsh {
            habits_file,
            log_file,
            habits: vec![],
            // entries: HashMap::new(),
        }
    }
}





fn load_entries() {

}

fn build_spark() {

}

fn build_consistency() {

}


