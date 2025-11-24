#[derive(Debug, Clone, Copy)]
pub enum Action {
    Move(i32, i32),

    ToggleInventory,
    ToggleInvTab,   // NEW: T/t cycles inventory tab

    InventoryUp,
    InventoryDown,
    UseConsumable, // also unequip when hovering sword/shield

    ToggleStats,

    Confirm,
    Interact,
    Choice(char),

    Quit, // Ctrl+C / Ctrl+Q
    None,
}
