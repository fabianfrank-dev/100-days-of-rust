use std::fs::File;
use std::fs;
use std::io::prelude::*;

 #[derive(Clone)]
struct Note{
    title: String,
    content: String
}

enum Action{
    Create(Note),
    View(String),
    Delete(String)
}

impl Action{
    fn execute(&self, notes: &mut Vec<Note>){   
        match self{
            Action::Create(note) => {
                let file = create_file(note.title.to_string());
                let _ = write_into_file(file, &note.content.as_bytes());
                println!("Note {} was successfully added", note.title);
 
            },
            Action::View(filename) => {
                open_file(filename.to_string());
            },
            Action::Delete(filename) => {
                let _ = delete_file(filename.to_string());
                println!("File was successfully deleted");
            },   
        }
    }
}

fn main(){
    let note = Note{
        title: "Test".to_string(),
        content: "Hello from test".to_string()
    };

    let mut notes = Vec::new();

    let create = Action::Create(note);
    create.execute(&mut notes);
}

fn create_file(filename: String) -> std::io::Result<File> {
    // create file
    let path: String = format!("notes/{}.txt", filename);
    let file: std::io::Result<File> = File::create(path);

    // return file
    file
}

fn open_file(filename: String){
    
    let path: String = format!("notes/{}.txt", filename);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("{}", contents);
}

fn write_into_file(file: std::io::Result<File>, content: &[u8]) -> std::io::Result<()>{
    let _ = file?.write_all(content);
    
    Ok(())
}

fn delete_file(filename: String) -> std::io::Result<()>{
    let path: String = format!("notes/{}.txt", filename);
    fs::remove_file(path)?;
    Ok(())
}
