use gdnative::api::{PhysicsBody, Node, RayCast};
use gdnative::core_types::Variant;
use gdnative::prelude::*;
use std::collections::HashMap;

pub struct InventoryItem {
    name: String,
    qty: i32,
    desc: String,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Inventory {
    storage: HashMap<String, InventoryItem>,
}

#[methods]
impl Inventory {
    fn new(_owner: &Node) -> Self {
        Inventory {
            storage: HashMap::new(),
        }
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        let camera_focus = unsafe {owner.get_node_as::<RayCast>("../Camera/Focus").unwrap()};
        camera_focus.connect(
            "picked_up_item",
            owner,
            "on_picked_up_item",
            VariantArray::new_shared(),
            1
        );
    }

    #[export]
    fn on_picked_up_item(&mut self, _owner: &Node, picked_item: Ref<PhysicsBody>) {
        godot_print!("{}", "Picking up item");
        let picked_item = unsafe { picked_item.assume_safe() };
        let item_id = picked_item.get("id");

        let qty = picked_item.get("qty").to_string();
        let qty = match qty.as_str() {
            "Null" => 1,
            _ => qty.to_string().parse::<i32>().unwrap(),
        };

        if self.storage.contains_key(&item_id.to_string()) {
            let inventory_item = self.storage.get_mut(&item_id.to_string()).unwrap();
            inventory_item.qty += qty;
        } else {
            let inventory_item = InventoryItem {
                name: picked_item.get("name_label").to_string(),
                desc: picked_item.get("desc_label").to_string(),
                qty: qty,
            };
            self.storage.insert(item_id.to_string(), inventory_item);
        };

        godot_print!("{:?}", self.storage.get(&item_id.to_string()).unwrap().qty);

    }
}
