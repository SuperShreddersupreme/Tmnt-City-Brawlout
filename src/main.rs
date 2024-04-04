use rand::Rng;
use std::io;

// Define struct for each character
struct Character {
    name: String,
    health: i32,
    attack_power: i32,
    defense: i32,
}

impl Character {
    fn new(name: &str, health: i32, attack_power: i32, defense: i32) -> Character {
        Character {
            name: String::from(name),
            health,
            attack_power,
            defense,
        }
    }

    fn attack(&self, other: &mut Character) {
        let damage = self.attack_power - other.defense;
        if damage > 0 {
            other.health -= damage;
        }
        println!("{} attacks {} for {} damage!", self.name, other.name, damage);
    }
}

fn main() {
    // Create Turtles and Enemies
    let mut turtles = vec![
        Character::new("Leonardo", 100, 20, 10),
        Character::new("Donatello", 100, 15, 15),
        Character::new("Michelangelo", 100, 25, 5),
        Character::new("Raphael", 100, 18, 12),
    ];

    let mut enemies = vec![
        Character::new("Shredder", 150, 30, 20),
        Character::new("Bebop", 120, 25, 15),
        Character::new("Rocksteady", 130, 28, 18),
    ];

    println!("Welcome to Teenage Mutant Ninja Turtles: City Brawlout!");
    println!("Choose your turtle:");

    // Display turtle selection menu
    for (i, turtle) in turtles.iter().enumerate() {
        println!("{}. {}", i + 1, turtle.name);
    }

    // Select player's turtle
    let mut player_index = String::new();
    io::stdin().read_line(&mut player_index).expect("Failed to read line");
    let player_index: usize = player_index.trim().parse().expect("Please enter a number");
    let player_index = player_index - 1; // Adjust index

    println!("You've chosen {}!", turtles[player_index].name);

    // Main game loop
    loop {
        // Display enemies
        println!("Enemies ahead:");
        for (i, enemy) in enemies.iter().enumerate() {
            println!("{}. {}: {} HP", i + 1, enemy.name, enemy.health);
        }

        // Select enemy to attack
        println!("Select enemy to attack:");
        let mut enemy_index = String::new();
        io::stdin().read_line(&mut enemy_index).expect("Failed to read line");
        let enemy_index: usize = enemy_index.trim().parse().expect("Please enter a number");
        let enemy_index = enemy_index - 1; // Adjust index

        // Player attacks enemy
        turtles[player_index].attack(&mut enemies[enemy_index]);

        // Check if enemy is defeated
        if enemies[enemy_index].health <= 0 {
            println!("{} defeated {}!", turtles[player_index].name, enemies[enemy_index].name);
            enemies.remove(enemy_index);
        }

        // Check if all enemies are defeated
        if enemies.is_empty() {
            println!("Congratulations! You've defeated all enemies!");
            break;
        }

        // Enemies attack back
        for enemy in enemies.iter_mut() {
            let target_index = rand::random::<usize>() % turtles.len(); // Randomly choose target turtle
            enemy.attack(&mut turtles[target_index]);
        }

        // Check if player's turtles are defeated
        if turtles.iter().all(|turtle| turtle.health <= 0) {
            println!("Game over! All turtles are defeated!");
            break;
        }
    }
}
