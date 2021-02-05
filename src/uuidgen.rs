
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid);
    Ok(())
}
