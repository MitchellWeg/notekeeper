mod note;
mod db;

use clap::Parser;
use chrono::NaiveDate;
use prettytable::{Table, Row, row, Cell};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // -s
    #[arg(short, long)]
    subject: Option<String>,

    // -c 
    #[arg(short, long)]
    content: Option<String>,

    // -d 
    #[arg(short, long)]
    due_date: Option<NaiveDate>,

    // -c 
    #[arg(short, long)]
    edit: Option<i32>,

    // -l
    #[arg(short, long)]
    list: bool,
}


fn main() {
    let args = Args::parse();
    let mut note_subject: String = String::from("NULL");
    let mut due_date: String = String::from("NULL");

    if args.list {
        list();
        return;
    }

    // Note that subject might be empty.
    if let Some(s) = &args.subject {
        note_subject = s.to_string();
    }


    if let Some(d) = &args.due_date {
        let parsed_date = d.to_string();        
        
        due_date = parsed_date;
    }

    let note_content = args.content.unwrap();

    let parsed = note::parse_note(
        note_subject,
        note_content,
        due_date,
    );

    db::save_note(parsed);
}

fn list() {
    let _notes = db::show();

    let notes = match _notes {
        Ok(n) => n,
        Err(e) => panic!("{}", e.to_string())
    };

    let mut table = Table::new();

    table.add_row(row!["Id", "Subject", "Content", "Due Date", "Inserted At"]);

    for note in notes {
        table.add_row(Row::new(vec![
            Cell::new(&note.id.unwrap().to_string()),
            Cell::new(&note.subject),
            Cell::new(&note.content),
            Cell::new(&note.due_date),
            Cell::new(&note.inserted_at.unwrap())]));
    }

    table.printstd();
}

