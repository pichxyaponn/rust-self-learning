fn main() {
    let mut treasure = String::from("gold coins");

    let frodo = &treasure;
    println!("Frodo sees: {}", frodo);
    let sam = &treasure;
    println!("Sam sees: {}", sam);
    
    // Important: Immutable references (frodo and sam) จะไม่ถูกใช้อีกต่อไปหลังจากจุดนี้
    // ดังนั้นจึงปลอดภัยที่จะสร้าง mutable reference ได้แล้ว

    let gandalf = &mut treasure;
    gandalf.push_str(" and silver coins");
    println!("Gandalf updates: {}", gandalf);

    println!("Final treasure: {}", treasure);
}
