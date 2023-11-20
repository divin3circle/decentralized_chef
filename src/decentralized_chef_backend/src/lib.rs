#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the Recipe structure
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Recipe {
    id: u64,
    name: String,
    category: String,
    ingredients: Vec<String>,
    instructions: String,
}

impl Storable for Recipe {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Recipe {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// New thread-local variables for our Recipe

thread_local! {
    static RECIPE_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static RECIPE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(RECIPE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for Recipe items")
    );

    static RECIPE_STORAGE: RefCell<StableBTreeMap<u64, Recipe, Memory>> =
        RefCell::new(StableBTreeMap::init(
            RECIPE_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Function to add a recipe
#[ic_cdk::update]
fn add_recipe(recipe: Recipe) -> Option<Recipe> {
    let id = RECIPE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .unwrap_or(0);

    let recipe = Recipe {
        id,
        name: recipe.name.clone(),
        category: recipe.category.clone(),
        ingredients: recipe.ingredients.clone(),
        instructions: recipe.instructions.clone(),
    };

    RECIPE_STORAGE.with(|service| service.borrow_mut().insert(recipe.id, recipe.clone()));
    Some(recipe)
}

// Function to search for recipes by category
#[ic_cdk::query]
fn search_by_category(category: String) -> Vec<Recipe> {
    RECIPE_STORAGE.with(|recipe_storage| {
        let recipe_storage = recipe_storage.borrow();

        let recipes: Vec<Recipe> = recipe_storage
            .iter()
            .filter_map(|(_, recipe)| {
                if recipe.category == category {
                    Some(recipe.clone())
                } else {
                    None
                }
            })
            .collect();
        return recipes;
    })
}
// Function to search for recipes by name
#[ic_cdk::query]
// Function to search for recipes by name
fn search_by_name(name: String) -> Option<Recipe> {
    RECIPE_STORAGE.with(|recipe_storage| {
        let recipe_storage = recipe_storage.borrow();

        let recipe = recipe_storage.iter().find_map(|(_, recipe)| {
            if recipe.name == name {
                Some(recipe.clone())
            } else {
                None
            }
        });
        return recipe;
    })
}

// Function to get all recipes
#[ic_cdk::query]
fn get_all_recipes() -> Vec<Recipe> {
    RECIPE_STORAGE.with(|recipe_storage| {
        let recipe_storage = recipe_storage.borrow();
        let recipe = recipe_storage
            .iter()
            .map(|(_, item)| item.clone())
            .collect();
        return recipe;
    })
}

ic_cdk::export_candid!();