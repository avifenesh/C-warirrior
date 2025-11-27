use super::constants::TILE_SIZE;
use super::player::Direction;
use super::state::Position;
use super::world::{TileType, World};

/// Calculate new position after moving in a direction
pub fn calculate_movement(current_pos: Position, direction: Direction, distance: f32) -> Position {
    match direction {
        Direction::Up => Position::new(current_pos.x, current_pos.y - distance),
        Direction::Down => Position::new(current_pos.x, current_pos.y + distance),
        Direction::Left => Position::new(current_pos.x - distance, current_pos.y),
        Direction::Right => Position::new(current_pos.x + distance, current_pos.y),
    }
}

/// Check if a position (in pixels) is walkable in the world
pub fn is_position_walkable(world: &World, position: Position) -> bool {
    let (tile_x, tile_y) = position.tile_coords(TILE_SIZE);

    // Check bounds
    if tile_x < 0 || tile_y < 0 {
        return false;
    }

    world.is_walkable(tile_x as usize, tile_y as usize)
}

/// Check if movement from one position to another would collide with walls
pub fn check_collision(world: &World, _from: Position, to: Position) -> bool {
    // Check if the destination is walkable
    if !is_position_walkable(world, to) {
        return true; // Collision detected
    }

    // Check the four corners of the player's bounding box (assuming player is smaller than tile)
    let player_size = TILE_SIZE * 0.8; // Player is 80% of tile size
    let half_size = player_size / 2.0;

    let corners = [
        Position::new(to.x - half_size, to.y - half_size), // Top-left
        Position::new(to.x + half_size, to.y - half_size), // Top-right
        Position::new(to.x - half_size, to.y + half_size), // Bottom-left
        Position::new(to.x + half_size, to.y + half_size), // Bottom-right
    ];

    // If any corner is not walkable, there's a collision
    for corner in &corners {
        if !is_position_walkable(world, *corner) {
            return true;
        }
    }

    false // No collision
}

/// Find the nearest interactable tile (terminal, door, etc.) within interaction range
pub fn find_nearest_interactable(
    world: &World,
    player_pos: Position,
    max_distance: f32,
) -> Option<(usize, usize, TileType)> {
    let (player_tile_x, player_tile_y) = player_pos.tile_coords(TILE_SIZE);
    let tile_range = (max_distance / TILE_SIZE).ceil() as i32;

    let mut nearest: Option<(usize, usize, TileType, f32)> = None;

    // Search in a square around the player
    for dy in -tile_range..=tile_range {
        for dx in -tile_range..=tile_range {
            let check_x = player_tile_x + dx;
            let check_y = player_tile_y + dy;

            if check_x < 0 || check_y < 0 {
                continue;
            }

            let check_x = check_x as usize;
            let check_y = check_y as usize;

            if let Some(tile) = world.get_tile(check_x, check_y) {
                if tile.interactable {
                    // Calculate distance
                    let tile_center_x = check_x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                    let tile_center_y = check_y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

                    let dx = tile_center_x - player_pos.x;
                    let dy = tile_center_y - player_pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance <= max_distance {
                        if let Some((_, _, _, current_dist)) = nearest {
                            if distance < current_dist {
                                nearest = Some((check_x, check_y, tile.tile_type, distance));
                            }
                        } else {
                            nearest = Some((check_x, check_y, tile.tile_type, distance));
                        }
                    }
                }
            }
        }
    }

    nearest.map(|(x, y, tile_type, _)| (x, y, tile_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::Tile;

    #[test]
    fn test_calculate_movement() {
        let pos = Position::new(100.0, 100.0);
        let new_pos = calculate_movement(pos, Direction::Up, 10.0);
        assert_eq!(new_pos.y, 90.0);
        assert_eq!(new_pos.x, 100.0);
    }

    #[test]
    fn test_is_position_walkable() {
        let world = World::new(10, 10);
        let pos = Position::new(32.0, 32.0); // Tile (1, 1)
        assert!(is_position_walkable(&world, pos));
    }

    #[test]
    fn test_collision_with_wall() {
        let mut world = World::new(10, 10);
        // Add a wall at (2, 2)
        world.tiles[2][2] = Tile::wall();

        let from = Position::new(32.0, 32.0); // Tile (1, 1)
        let to = Position::new(64.0, 64.0); // Tile (2, 2) - wall

        assert!(check_collision(&world, from, to));
    }

    #[test]
    fn test_no_collision_with_floor() {
        let world = World::new(10, 10);

        let from = Position::new(32.0, 32.0); // Tile (1, 1)
        let to = Position::new(96.0, 96.0); // Tile (3, 3) - floor

        assert!(!check_collision(&world, from, to));
    }
}
