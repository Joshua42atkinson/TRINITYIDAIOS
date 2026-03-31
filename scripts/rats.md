# Rats

## Overview
Rats are small, omnivorous rodents that are often found in large groups. They are known for their intelligence, adaptability, and ability to thrive in various environments, including urban and rural settings. Rats are also commonly used as lab animals in scientific research due to their fast reproduction rates and genetic similarities to humans.

## Characteristics
- **Size**: Typically 9–11 inches in length (body + tail)
- **Weight**: 1–1.5 lbs depending on species and diet
- **Lifespan**: 1–3 years in the wild, up to 6 years in captivity
- **Diet**: Omnivorous — eats grains, fruits, vegetables, and small insects
- **Behavior**: Highly social, nocturnal, and known for burrowing and gnawing
- **Reproduction**: Can produce 6–12 offspring per litter, with multiple litters per year

## Habitat
- **Natural Habitat**: Rats are found in a variety of environments, including forests, grasslands, and deserts. They adapt well to human settlements.
- **Urban Habitat**: Commonly inhabit sewers, basements, and attics. They are often associated with unsanitary conditions but can thrive in clean environments too.

## Role in Folklore and Mythology
- **Trash Panda**: Often linked to negative stereotypes due to their association with garbage and disease.
- **Lab Rat**: In popular culture, rats are used as symbols for scientific experiments, often depicted as test subjects in labs.
- **Intelligence**: Some rat characters are portrayed as clever, resourceful, or even cunning, such as in stories like *The Rats of NIMH* or *Pied Piper of Hamelin*.

## Role in Gaming
- **Enemies**: Rats can serve as weak but numerous enemies in games, often swarming the player.
- **Minions**: Used as minions or minibosses in dungeon crawlers or survival games.
- **Companions**: Occasionally, rats are depicted as loyal companions or familiars, such as in *Castlevania* or *Dark Souls*.
- **Stealth Mechanics**: Rats can be used to teach stealth mechanics due to their small size and agility.
- **Resource Gathering**: In survival or crafting games, rats can be hunted for food or other resources.

## Game Mechanics
- **Swarm Behavior**: Rats can be programmed to move in groups, following a leader or reacting to player presence.
- **Health and Damage**: Weak enemies with low health but high attack power in groups.
- **AI**: Simple AI for movement and pathfinding, but can be enhanced with dynamic behaviors like fleeing or attacking based on player actions.
- **Spawn System**: Rats can spawn dynamically in alleys, sewers, or abandoned buildings to create a sense of realism and challenge.
- **Drop System**: Potential to drop small items like food scraps or health potions when killed.

## Example Code Snippet (Rust)
```rust
// Define a simple Rat struct for game enemies
struct Rat {
    health: u32,
    position: (f32, f32),
    is_aggressive: bool,
}

impl Rat {
    // Create a new rat with default values
    fn new() -> Self {
        Rat {
            health: 10,
            position: (0.0, 0.0),
            is_aggressive: false,
        }
    }

    // Spawn a rat at a specific position
    fn spawn(x: f32, y: f32) -> Self {
        Rat {
            health: 10,
            position: (x, y),
            is_aggressive: true,
        }
    }

    // Basic AI for rat movement
    fn update_position(&mut self, target: (f32, f32)) {
        if self.is_aggressive {
            // Move toward the target (e.g., player)
            self.position.0 += (target.0 - self.position.0).signum() * 0.5;
            self.position.1 += (target.1 - self.position.1).signum() * 0.5;
        } else {
            // Move away from the target (e.g., player)
            self.position.0 -= (target.0 - self.position.0).signum() * 0.5;
            self.position.1 -= (target.1 - self.position.1).signum() * 0.5;
        }
    }
}
```

## Example Game Loop
```rust
// Simple game loop to simulate rat swarm behavior
fn game_loop(mut rats: Vec<Rat>, player_position: (f32, f32)) -> Vec<Rat> {
    for rat in rats.iter_mut() {
        rat.update_position(player_position);
        // Check for collisions or attacks
        if is_colliding(rat.position, player_position) {
            println!("Rat attacks the player!");
        }
    }
    rats
}

fn main() {
    let mut rats = Vec::new();
    rats.push(Rat::spawn(10.0, 20.0));
    rats.push(Rat::spawn(15.0, 25.0));
    rats.push(Rat::spawn(5.0, 5.0));

    let player_position = (0.0, 0.0);
    let updated_rats = game_loop(rats, player_position);
}
```

## Integration with Trinity
- **Rat Entity**: Integrate the `Rat` struct as a game entity in Trinity.
- **Spawn System**: Use the `spawn` method to dynamically spawn rats in-game.
- **AI System**: Implement the `update_position` method for rat movement and behavior.

## Next Steps
- Define a Bevy-compatible entity for rats.
- Implement a spawn system for dynamic rat generation.
- Add collision detection and swarm behavior.

[CONTINUE]