mod tasks;
mod guessno;

fn main() {
    let myname = String::from("immutable reference");
    let mut firstname = String::from("Rohan");

    tasks::run(myname);
    tasks::append_text(&mut firstname);
    guessno::guess_number();
}
