// storage with weight (kg) limit
// 6 slots for equippables that do not count towards the limit
// 
// items have a weight
// a durability
// a category

use rand::random_range;

pub enum InventoryError {
    NotEnoughStorage {
        quantity_not_added: u32,
    },
    NonExistantIdx(usize),
    NoItemEquipped(EquipmentSlot),
}


pub struct ActorInventory {
    pub storage: Vec<(InvItem, u32)>,
    pub max_storage_weight: u32,
    pub slots: ActorSlots,
}

impl ActorInventory {
    pub fn current_weight(&self) -> u32 {
        self.storage.iter().fold(0, |acc, items| items.0.weight * items.1 + acc )
    }

    /// If there is not enough capacity for any quantity of items, 
    /// addition of items occurs up to the permitted quantity, 
    /// and an error is returned specifying the quantity of items that could not be added. 
    pub fn add_item(&mut self, item: InvItem, quantity: u32) -> Result<(), InventoryError> {
        let avaliable_weight = self.max_storage_weight - self.current_weight();
        let quantity_that_fits = avaliable_weight / item.weight;
        if quantity_that_fits == 0 { return Err(InventoryError::NotEnoughStorage { quantity_not_added: quantity })}
        let valid_quantity = if quantity_that_fits < quantity { quantity_that_fits } else { quantity };

        match self.storage.iter_mut().find(|items| items.0 == item  ) {
            Some(item) => item.1 += valid_quantity,
            None => self.storage.push((item, valid_quantity)),
        }

        if valid_quantity < quantity { 
            Err(InventoryError::NotEnoughStorage { quantity_not_added: quantity - valid_quantity })
        } else { Ok(()) }
    }

    /// Upon succesful removal, returns the item and quantity removed
    /// if attempting to remove too much, it will remove up to the limit
    /// 
    /// Err if idx is out of bounds, returning the idx
    pub fn remove_item_idx(&mut self, idx: usize, quantity: u32) -> Result<(InvItem, u32), InventoryError> {
        if idx < self.storage.len() {
            let target_item_quantity = self.storage[idx].1;

            let (item, quantity) = 
                if quantity < target_item_quantity { 
                    self.storage[idx].1 -= quantity;
                    (self.storage[idx].0.clone(), quantity)
                } else {
                    self.storage.remove(idx)
                };

            Ok((item, quantity))
        } else {
            Err(InventoryError::NonExistantIdx(idx))
        }
    }

    pub fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<InvItem, InventoryError> {
        let target_slot = self.slots.get_mut_slot(slot);

        if let Some(item) = target_slot.clone() {
            *target_slot = None;
            self.add_item(item.clone(), 1).map(|_| item )
        } else { 
            Err(InventoryError::NoItemEquipped(slot))
        }
    }

    pub fn equip_item(&mut self, slot: EquipmentSlot, item: InvItem) -> Result<Option<InvItem>, InventoryError> {
        let result = if self.slots.get_ref_slot(slot).is_some() {
            self.unequip_item(slot).map(|i| Some(i) )
        } else {
            Ok(None)
        };
        
        let target_slot = self.slots.get_mut_slot(slot); 
        *target_slot = Some(item);

        result
    }
}

#[derive(Default)]
pub struct ActorSlots {
    pub front: Option<InvItem>,
    pub back: Option<InvItem>,
    pub left: Option<InvItem>,
    pub right: Option<InvItem>,
    pub top: Option<InvItem>,
    pub bottom: Option<InvItem>,
}

impl ActorSlots {
    pub fn as_slice(&self) -> [&Option<InvItem>; 6] {
        [
            &self.front,
            &self.back,
            &self.left,
            &self.right,
            &self.top,
            &self.bottom
        ]
    } 

    pub fn get_mut_slot(&mut self, slot: EquipmentSlot) -> &mut Option<InvItem> {
        match slot {
            EquipmentSlot::Front => &mut self.front,
            EquipmentSlot::Back => &mut self.back,
            EquipmentSlot::Right => &mut self.right,
            EquipmentSlot::Left => &mut self.left,
            EquipmentSlot::Top => &mut self.top,
            EquipmentSlot::Bottom => &mut self.bottom,
        }
    }

    pub fn get_ref_slot(&self, slot: EquipmentSlot) -> &Option<InvItem> {
        match slot {
            EquipmentSlot::Front => &self.front,
            EquipmentSlot::Back => &self.back,
            EquipmentSlot::Right => &self.right,
            EquipmentSlot::Left => &self.left,
            EquipmentSlot::Top => &self.top,
            EquipmentSlot::Bottom => &self.bottom,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom
}

impl EquipmentSlot {
    pub fn all_sides() -> Vec<EquipmentSlot> {
        vec![EquipmentSlot::Left, EquipmentSlot::Right, EquipmentSlot::Front, EquipmentSlot::Back]
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct InvItem {
    pub name: InvItemName,
    pub categories: Vec<InvItemCategory>,
    pub valid_slots: Vec<EquipmentSlot>,
    pub integrity: u32, 
    pub weight: u32,
}

impl From<InvItemName> for InvItem {
    fn from(value: InvItemName) -> Self {
        match value {
            InvItemName::WoodenBuckler => InvItem {
                name: value,
                categories: vec![InvItemCategory::Shield],
                valid_slots: EquipmentSlot::all_sides(),
                integrity: 10,
                weight: 5,
            },
            InvItemName::StonePickaxe => InvItem {
                name: value,
                categories: vec![
                    InvItemCategory::DiggingTool,
                    InvItemCategory::Weapon { 
                        reach: 1, 
                        projectile: false, 
                        effects: Vec::new(), 
                        dmg: DiceNotation::new(5, 1), 
                    }
                ],
                valid_slots: EquipmentSlot::all_sides(),
                integrity: 8,
                weight: 10,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InvItemName {
    WoodenBuckler,
    StonePickaxe,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DiceNotation {
    pub sides: u32,
    pub num_of_rolls: u8
}

impl DiceNotation {
    pub fn new(sides: u32, num_of_rolls: u8) -> Self { DiceNotation { sides, num_of_rolls } }

    pub fn roll(&self) -> u32 {
        let mut accumulator = 0;
        for _ in 0..self.num_of_rolls {
            accumulator += random_range(1..=self.sides);
        } 
        accumulator
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemEffect {
    // penetration either stops at the max amount of penetratable enemies, or simply runs out of power with the decrease in power after each enemy hit
    Penetration {
        penetration_max: u8,
        pcnt_dmg_reduction_per_enemy: u8, 
    },
    Freeze {
        duration: u64
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum InvItemCategory {
    Weapon {
        // reach in squares
        reach: u8,
        // is it a projectile weapon
        projectile: bool,
        effects: Vec<ItemEffect>,
        dmg: DiceNotation
    },
    Shield,
    Mobility,
    DiggingTool,
}
