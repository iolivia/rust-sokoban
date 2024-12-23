use hecs::Entity;

#[derive(Debug)]
pub struct EntityMoved {
    pub entity: Entity,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when an entity is moved
    EntityMoved(EntityMoved),

    // Fired when the box is placed on a spot
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}
