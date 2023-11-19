use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use candid::CandidType;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Recipe {
    name: String,
    category: String,
    ingredients: Vec<String>,
    instructions: String,
}

#[derive(Default, CandidType)]
struct RecipeManager {
    recipes: HashMap<String, Recipe>,
    categories: HashMap<String, Vec<String>>,
}

lazy_static::lazy_static! {
    static ref RECIPE_MANAGER: Arc<RwLock<RecipeManager>> = Arc::new(RwLock::new(RecipeManager::default()));
}

fn sanitize_input(input: &str) -> String {
    // Replace any potentially harmful characters
    input.chars().filter(|&c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

#[ic_cdk::update]
fn add_recipe(name: String, category: String, ingredients: Vec<String>, instructions: String) -> Result<(), String> {
    let sanitized_name = sanitize_input(&name);

    let recipe_manager = RECIPE_MANAGER.write().map_err(|_| "Failed to acquire write lock")?;

    if recipe_manager.recipes.contains_key(&sanitized_name) {
        return Err(format!("Recipe with name '{}' already exists", sanitized_name));
    }

    let recipe = Recipe {
        name: sanitized_name.clone(),
        category: category.clone(),
        ingredients,
        instructions,
    };

    let mut categories = recipe_manager.categories.clone();
    categories
        .entry(category.clone())
        .or_insert_with(Vec::new)
        .push(sanitized_name.clone());

    let mut recipes = recipe_manager.recipes.clone();
    recipes.insert(sanitized_name, recipe);

    let mut recipe_manager = recipe_manager.clone();
    recipe_manager.categories = categories;
    recipe_manager.recipes = recipes;

    *RECIPE_MANAGER.write().map_err(|_| "Failed to acquire write lock")? = recipe_manager;

    Ok(())
}

#[ic_cdk::query]
fn search_by_category(category: String) -> Result<Vec<Recipe>, String> {
    let sanitized_category = sanitize_input(&category);

    let recipe_manager = RECIPE_MANAGER.read().map_err(|_| "Failed to acquire read lock")?;

    let recipe_names = recipe_manager.categories.get(&sanitized_category).cloned().unwrap_or_default();

    let recipes: Vec<Recipe> = recipe_names
        .into_iter()
        .filter_map(|name| recipe_manager.recipes.get(&name).cloned())
        .collect();

    Ok(recipes)
}

#[ic_cdk::query]
fn search_by_name(name: String) -> Result<Option<Recipe>, String> {
    let sanitized_name = sanitize_input(&name);

    let recipe_manager = RECIPE_MANAGER.read().map_err(|_| "Failed to acquire read lock")?;

    Ok(recipe_manager.recipes.get(&sanitized_name).cloned())
}

#[ic_cdk::query]
fn get_all_recipes() -> Result<Vec<Recipe>, String> {
    let recipe_manager = RECIPE_MANAGER.read().map_err(|_| "Failed to acquire read lock")?;

    let all_recipes: Vec<Recipe> = recipe_manager.recipes.values().cloned().collect();

    Ok(all_recipes)
}

ic_cdk::export_candid!();
