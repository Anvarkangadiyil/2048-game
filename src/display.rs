
//to print footer meassage under board
pub fn footer(){
    println!();
    println!("[ ← ↑ → ↓ ], q or ESC for quit\r\n");
    println!();
}

//to display lose message
pub fn to_display_lose(){
    println!();
    println!("You Lose 😥 Enter ESC or q for quit");
}

//to display won message
pub fn to_display_won(){
    println!();
    println!("🎉🎊You Won🥇(*^_^*)")

}

//to display score
pub fn to_display_score(score:i32){
    print!("\t\t\tScore:{}",score);
    println!();
    println!();

}