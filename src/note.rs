pub struct Note {
    pub id: Option<i32>,    
    pub subject: String,
    pub content: String,
    pub due_date: String,
    pub inserted_at: Option<String>,
}

pub fn parse_note(subject: String, content: String, due_date: String) -> Note {
    let parsed_note = Note {
        id: None,
        subject: subject,
        content: content,
        due_date: due_date,
        inserted_at: None,
    };

    return parsed_note;
}   
