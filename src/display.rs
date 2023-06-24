pub fn footer(){
    println!();
    println!("[ ← ↑ → ↓ ], q or ESC for quit\r\n");
    println!();
}

pub fn to_display_lose(){
    println!();
    println!("You Lose 😥 Enter ESC or q for quit");
}

pub fn to_display_won(){
    println!();
    println!("🎉🎊You Won🥇(*^_^*)")

}

pub fn to_display_score(score:i32){
    print!("\t\t\tScore:{}",score);
    println!();
    println!();

}