use rand::Rng;
use std::io;

// Enum to represent different character classes
#[derive(Debug, Clone, Copy)]
enum ClassType {
    Warrior,
    Mage,
    Rogue,
}

// Struct to represent a character in the game
#[derive(Debug)]
struct Character {
    name: String,
    class: ClassType,
    health: i32,
    attack: i32,
    potion_used: bool,
}

impl Character {
    // Constructor for creating a new character with randomized stats
    fn new(name: &str, class: ClassType) -> Self {
        let mut rng = rand::thread_rng();
        let base_health = rng.gen_range(80..121);
        let base_attack = rng.gen_range(15..26);

        // Apply class-specific stat modifiers
        let (health, attack) = match class {
            ClassType::Warrior => (base_health + 20, base_attack - 2),
            ClassType::Mage => (base_health - 10, base_attack + 5),
            ClassType::Rogue => (base_health, base_attack + 2),
        };

        Character {
            name: name.to_string(),
            class,
            health,
            attack,
            potion_used: false,
        }
    }

    // Check if character is still alive
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    // Apply damage to character
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        println!("{} takes {} damage! Health now: {}", self.name, amount, self.health);
    }

    // Attack another character with randomized damage
    fn attack_target(&self, target: &mut Character) {
        let mut rng = rand::thread_rng();
        let damage = rng.gen_range(self.attack - 5..=self.attack + 5);
        println!("{} attacks {} for {} damage!", self.name, target.name, damage);
        target.take_damage(damage);
    }

    // Heal character using a potion (once per game)
    fn heal(&mut self) {
        if self.potion_used {
            println!("Potion already used!");
        } else {
            let heal_amount = 30;
            self.health += heal_amount;
            self.potion_used = true;
            println!("{} uses a healing potion! Health restored to {}", self.name, self.health);
        }
    }

    // Return class name as string
    fn class_name(&self) -> &str {
        match self.class {
            ClassType::Warrior => "Warrior",
            ClassType::Mage => "Mage",
            ClassType::Rogue => "Rogue",
        }
    }
}

// Prompt user to choose a class
fn choose_class() -> ClassType {
    println!("Choose your class:");
    println!("1. Warrior (+HP, -ATK)");
    println!("2. Mage (-HP, +ATK)");
    println!("3. Rogue (+ATK)");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" => return ClassType::Warrior,
            "2" => return ClassType::Mage,
            "3" => return ClassType::Rogue,
            _ => println!("Invalid choice. Enter 1, 2, or 3."),
        }
    }
}

fn main() {
    println!("Welcome to Rusty DnD Battle!");

    // Get player name
    println!("Enter your character's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    // Class selection and character creation
    let class = choose_class();
    let mut player = Character::new(name, class);
    let mut monster = Character::new("Goblin", ClassType::Warrior);

    // Display initial stats
    println!("\nYour stats: {:?} ({})", player, player.class_name());
    println!("Monster stats: {:?} ({})", monster, monster.class_name());

    println!("\nBattle begins!");
    let mut turn = 1;

    // Main battle loop
    while player.is_alive() && monster.is_alive() {
        println!("\n--- Turn {} ---", turn);
        println!("Choose action: 1) Attack  2) Heal");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");

        match action.trim() {
            "1" => player.attack_target(&mut monster),
            "2" => player.heal(),
            _ => {
                println!("Invalid action. Defaulting to attack.");
                player.attack_target(&mut monster);
            }
        }

        // Monster attacks if still alive
        if monster.is_alive() {
            monster.attack_target(&mut player);
        }

        // Check for victory or defeat
        if !player.is_alive() {
            println!("{} has been defeated! {} wins!! Game Over!!", player.name, monster.name);
            break;
        } else if !monster.is_alive() {
            println!("{} has been defeated by {}!! You win!", monster.name, player.name);
            break;
        }

        turn += 1;
    }
}
