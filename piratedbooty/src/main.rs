use piratedbooty::PirateDBooty;


fn main() {
    let mut db = PirateDBooty::load("data.db");
    db.set("name".into(), "Dude".into());
    db.set("TheOnePiece".into(), "IsReal!!!".into());

    println!("name = {:?}", db.get("name"));
    println!("TheOnePiece = {:?}", db.get("TheOnePiece"));

    db.persist("data.db");
    
}
