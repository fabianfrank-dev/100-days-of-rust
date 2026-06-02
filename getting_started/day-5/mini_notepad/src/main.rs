 #[derive(Clone)]
struct Note{
    id: u32,
    title: String,
    content: String
}

enum Action{
    Create(Note),
    View(u32),
    Delete(u32)
}

impl Action{
    fn execute(&self, notes: &mut Vec<Note>){   
        match self{
            Action::Create(note) => {
                 notes.push(note.clone());
                 println!("Note (id: {} Title: {}) was successfully added", note.id, note.title);
             },
            Action::View(id) => {
                // iterate through the notes to find IDs
                let note_opt = notes.iter().find(|n| n.id == *id);
                // if note exists, return the note content
                match note_opt{
                    Some(note) => println!("{}" ,note.content),
                    _ => println!("Note not found"),
                }
            },
            Action::Delete(id) => {
                let note_opt = notes.iter().find(|n| n.id == *id);
                match note_opt{
                    Some(_) => { 
                        notes.retain(|n| n.id != *id);
                        println!("Deletion was successful");
                     }
                    _ => println!("Note Not Found"),
                };
            },   
        }
    }
}
fn main() {
    // create a note
    let note = Note{
        id: 1,
        title: "Hello".to_string(),
        content: "Hello".to_string(),
     };
    
     // create an action for the creation of note
    let create = Action::Create(note);
    
     // declare new vector to store note in
    let mut notes = Vec::new();
    
     // execute creation
    create.execute(&mut notes);
    
     //initialise view
    let view = Action::View(1);
    
     // execute view
    view.execute(&mut notes);
    
     // delete 
    let delete = Action::Delete(1);
    
     // execute deletion
    delete.execute(&mut notes);
    
     // check again
    view.execute(&mut notes);
}
