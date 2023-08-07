use std::io;
enum CharacterType {
    Wizard,
    Warrior,
}

struct Character {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

fn create_character(type_char: &CharacterType) -> Character {
    match type_char {
        CharacterType::Wizard => Character {
           name: String::from("Wizard"),
           hp: 60,
           atk: 90,
           def: 20,
        },
        CharacterType::Warrior => Character {
            name: String::from("Warrior"),
            hp: 90,
            atk: 60,
            def: 50,
        }
    }
}

fn create_enemy(type_char: CharacterType) -> Character {
    match type_char {
        CharacterType::Wizard => Character {
           name: String::from("Wizard (Enemy)"),
           hp: 60,
           atk: 90,
           def: 20,
        },
        CharacterType::Warrior => Character {
            name: String::from("Warrior (Enemy)"),
            hp: 90,
            atk: 60,
            def: 50,
        }
    }
}


fn main() {
    println!("\n{WELCOME}\n");

    println!("Please, choose your character:");
    println!("  1 - Wizard");
    println!("  2 - Warrior");

    let mut option_char = String::new();
    io::stdin().read_line(&mut option_char).expect("Please, choose a valid option.");
    let option_char: u32 = option_char.trim().parse().expect("Invalid option.");

    let char_type = match option_char {
        1 => { 
            println!("\n{WIZARD}\n"); 
            CharacterType::Wizard
        },
        2 => { 
            println!("\n{WARRIOR}\n");
            CharacterType::Warrior
        },
        _ => {
            println!("Invalid choice. Using Warrior as default...");
            CharacterType::Warrior
        }
    };

    let mut player = create_character(&char_type);
    let mut enemy = create_enemy(match char_type {
        CharacterType::Warrior => CharacterType::Wizard,
        CharacterType::Wizard => CharacterType::Warrior,
    });

    println!("These are your points:");
    println!("+----+--------+---------+");
    println!("| HP | ATTACK | DEFENSE |");
    println!("+----+--------+---------+");
    println!("| {} |   {}   |   {}    | ",
        player.hp, player.atk, player.def
    );
    println!("+----+--------+---------+");
    println!("");
    println!("You will fight against {}!", &enemy.name);
    println!("═══════════════════════════════════════════");
    loop {
        println!("Choose the action:");
        println!("1. to attack");
        println!("2. to defend");
        println!("3. exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Please, choose a valid option.");
        let option: u32 = option.trim().parse().expect("Invalid option");

        match option {
            1 => {
                let player_damage = player.atk - enemy.def;
                enemy.hp -= player_damage;
                println!("\n{ATTACK}\n"); 
                println!("You attacked the enemy for {} damage!", player_damage);

                println!("...");
                let enemy_damage = enemy.atk - player.def;
                player.hp -= enemy_damage;
                println!("Enemy attacked you and caused {} damage points", enemy_damage);

                if player.hp <= 0 {
                    println!("You have been defeated.");
                    println!("\n{GAMEOVER}\n"); 
                    println!("-------------------------");
                } else {
                    println!("You have won.");
                    println!("CONGRATULATIONS");
                    println!("-------------------------");
                }

            }
            2 => {
                println!("-------------------------");
                println!("You defended yourself!");

                let enemy_damage = enemy.atk / 2 - player.def;
                player.hp -= enemy_damage;
                println!("Enemy attacked you and caused {} damage points", enemy_damage);

                if player.hp <= 0 {
                    println!("You have been defeated.");
                    println!("\n{GAMEOVER}\n");
                    println!("-------------------------");
                    break;
                }
            }
            3 => {
                println!("See you!");
                break;
            }
            _ => {
                println!("Invalid option. Try again");
            }
        }
    }

}


const WELCOME: &str = r#"welcome to...
═══════════════════════════════════════════
══════════⊹⊱≼ BATTLE SIMULATOR ≽⊰⊹══════════
═══════════════════════════════════════════"#;

const WIZARD: &str = r#"YOU CHOSE WIZARD...
_,._      
 .||,       /_ _\\     
\.`',/      |'L'| |    
= ,. =      | -,| L    
/ || \    ,-'\"/,'`.   
  ||     ,'   `,,. `.  
  ,|____,' , ,;' \| |  
(3|\    _/|/'   _| |  
  ||/,-''  | >-'' _,\\ 
  ||'      ==\ ,-'  ,' 
  ||       |  V \ ,|   
  ||       |    |` |   
  ||       |    |   \  
  ||       |    \    \ 
  ||       |     |    \
  ||       |      \_,-'
  ||       |___,,--")_\
  ||         |_|   ccc/
  ||        ccc/       
  ||                
"#;


const WARRIOR: &str = r#"YOU CHOSE WARRIOR...
|\             //
 \\           _!_
  \\         /___\
   \\        [+++]
    \\    _ _\^^^/_ _
     \\/ (    '-'  ( )
     /( \/ | {&}   /\ \
       \  / \     / _> )
        "`   >:::;-'`""'-.
            /:::/         \
           /  /||   {&}   |
          (  / (\         /
          / /   \'-.___.-'
        _/ /     \ \
       /___|    /___|
"#;

const ATTACK: &str = r#"
        /
*//////{<>==================-
        \
"#;

const GAMEOVER: &str = r#"

      ,-=-.      
     /  +  \     >--->  GAME OVER >--->
     | ~~~ |  
     |R.I.P|      
\vV,,|_____|V,//_____/VvV,v,|_|/,,vhjwv/,
"#;