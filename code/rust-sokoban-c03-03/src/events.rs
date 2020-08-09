// ANCHOR: structs
pub type EntityId = u32;

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}
// ANCHOR_END: structs

// ANCHOR: event_enum
#[derive(Debug)]
pub enum Event {
    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when an entity is moved
    EntityMoved(EntityMoved),

    // Fired when the box is placed on a spot
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}
// ANCHOR_END: event_enum
