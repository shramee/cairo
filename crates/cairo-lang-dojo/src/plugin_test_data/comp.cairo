#[contract]
mod PositionComponent {

    struct Storage {
        world_address: felt,
        state: Map::<felt, Position>,
    }

    // Initialize PositionComponent.
    #[external]
    fn initialize(world_addr: felt) {
        let world = world_address::read();
        assert(world == 0, 'PositionComponent: Already initialized.');
        world_address::write(world_addr);
    }

    // Set the state of an entity.
    #[external]
    fn set(entity_id: felt, value: Position) {
        state::write(entity_id, value);
    }

    // Get the state of an entity.
    #[view]
    fn get(entity_id: felt) -> Position {
        return state::read(entity_id);
    }

    #[view]
    fn is_zero(entity_id: felt) -> bool {
        let self = state::read(entity_id);
        let res:u128 = self.x-self.y;
        let res:felt = u128_to_felt(res);
        match res {
            0 => bool::True(()),
            _ => bool::False(()),
        }
    }

    #[view]
    fn is_equal(entity_id: felt, b: Position) -> bool {
        let self = state::read(entity_id);
        self.x == b.x & self.y == b.y
    }
}
