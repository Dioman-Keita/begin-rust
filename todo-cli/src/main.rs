use std::{io};

struct Task {
    id: u32,
    content: String,
}

fn read_buffer(base: &mut Vec<Task>) {
    let mut task_description: String = String::new();
    io::stdin().read_line(&mut task_description).expect("Erreur de lecture");
    let final_task = Task { id: (base.len() + 1) as u32, content: task_description.trim().to_string() };
   base.push(final_task);
}

fn list_task(base: &[Task])  {
    for task in base {
        println!("{} - {}", task.id, task.content);
    }
}

fn show_menu(user_option: &mut String) {

    println!("======== CHOISISSEZ UNE OPTION ==========");
    println!("1. Ajouter des tache");
    println!("2. Lister");
    println!("3. Quitter");
    io::stdin().read_line(user_option).expect("Valeur incorrect");

}


fn main() {

    let mut base: Vec<Task> = vec![];

    loop {
        let mut user_option: String = String::new();
    
        show_menu(&mut user_option);

        match user_option.trim() {
            "1" => read_buffer(&mut base),
            "2" => list_task(&base),
            "3" => break,
            _ => println!("Option non valide")
        }

    }
}
