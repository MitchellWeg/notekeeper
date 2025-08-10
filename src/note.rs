pub struct Note {
    pub subject: String,
    pub content: String,
    pub due_date: String,
    pub inserted_at: Option<String>,
}

pub fn parse_note(subject: String, content: String, due_date: String) -> Note {
    let parsed_note = Note {
        subject: subject,
        content: content,
        due_date: due_date,
        inserted_at: None,
    };

    return parsed_note;
}   
