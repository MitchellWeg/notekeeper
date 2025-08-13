mod note;
mod db;

use clap::Parser;
use chrono::NaiveDate;
use prettytable::{Table, Row, row, Cell};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    // -f
    #[arg(short, long)]
    filter: Option<String>,

    #[arg()]
    rest: Vec<String>,
}


fn main() {
    let args = Args::parse();
    let mut due_date: String = String::from("NULL");

    if args.list {
        list(args.filter);
        return;
    }

    if let Some(d) = &args.due_date {
        let parsed_date = d.to_string();        
        
        due_date = parsed_date;
    }

    let note_content = if args.rest.len() > 0 {
        args.rest.get(0).unwrap().to_string()
    }
    else {
        args.content.unwrap()
    };


    let parsed = note::parse_note(
        note_content,
        due_date,
    );

    db::save_note(parsed);
}

fn list(filter: Option<String>) {
    let _notes = db::show(filter);

    let notes = match _notes {
        Ok(n) => n,
        Err(e) => panic!("{}", e.to_string())
    };

    let mut table = Table::new();

    table.add_row(row!["Id", "Content", "Due Date", "Inserted At"]);

    for note in notes {
        table.add_row(Row::new(vec![
            Cell::new(&note.id.unwrap().to_string()),
            Cell::new(&note.content),
            Cell::new(&note.due_date),
            Cell::new(&note.inserted_at.unwrap())]));
    }

    table.printstd();
}

