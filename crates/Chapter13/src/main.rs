fn main() {
    let p;
    {
        let q = Appellation {
            name: "Cardamiie hirsuta".to_string(),
            nicknames: vec!["shotweed".to_string(), "bittercress".to_string()],
        };
        if complicated_condition() {
            p = q;
        }
    }
    println!("Sproing! What was that?");
}

fn complicated_condition() -> bool {
    false
}
struct Appellation {
    name: String,
    nicknames: Vec<String>,
}
impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}
