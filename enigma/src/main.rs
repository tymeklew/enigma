use enigma_tui::{Colour, Modifier, Style};
fn main() {
    println!("{}", "real".style(Some(Colour::Red), None));
}

/*
Word : _ e _ _ _ _ _

guess the word

Word : f e _ _ _ _ _





Host or Join

# Host
username : Username
config : Config{
    max_players : 10,
    words : 20,
    points_scale : f32 = 1.4
}
port : 8080

Waiting for players ...
Jerome has joined
Apple has joined


[Start Game] [Exit]


#Join
Enter Ip : 127.0.0.1:8080
username : String
See Log



# In game
                          _ e _ _ _ _ _
Players                                                             Chat
Jerome               Jeome Guessed correctly and gained 15pts       Jerome has left
Apple                Next word                                      Jerome : Kys now
Tymek


[Leave]


# Afer Game
1st Place : Jerome
2nd Place : Apple,
3rd Place : Tymek

#Host
[Again]

#Player
[Play again]

*/
