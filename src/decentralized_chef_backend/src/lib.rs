use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use candid::CandidType;

// Define the Recipe structure
#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Recipe {
    name: String,
    category: String,
    ingredients: Vec<String>,
    instructions: String,
}

// Define a RecipeManager struct to manage recipes
#[derive(Default, CandidType, Deserialize, Serialize)]
struct RecipeManager {
    recipes: HashMap<String, Recipe>,
    categories: HashMap<String, Vec<String>>,
}

// Function to add a recipe
#[ic_cdk::update]
fn add_recipe(name: String, category: String, ingredients: Vec<String>, instructions: String) {
    let recipe = Recipe {
        name: name.clone(),
        category: category.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
    };

    let mut recipe_manager = RecipeManager::default();

    if !recipe_manager.recipes.contains_key(&name) {
        recipe_manager.recipes.insert(name.clone(), recipe.clone());

        // Update categories map
        if !recipe_manager.categories.contains_key(&category) {
            recipe_manager.categories.insert(category.clone(), vec![name.clone()]);
        } else {
            recipe_manager.categories.get_mut(&category).unwrap().push(name.clone());
        }
    }
}

// Function to search for recipes by category
#[ic_cdk::query]
fn search_by_category(category: String) -> Vec<Recipe> {
    let recipe_manager = RecipeManager::default();

    if let Some(recipe_names) = recipe_manager.categories.get(&category) {
        let recipes: Vec<Recipe> = recipe_names
            .iter()
            .filter_map(|name| recipe_manager.recipes.get(name))
            .cloned()
            .collect();
        return recipes;
    }

    Vec::new()
}

// Function to search for recipes by name
#[ic_cdk::query]
fn search_by_name(name: String) -> Option<Recipe> {
    let recipe_manager = RecipeManager::default();
    recipe_manager.recipes.get(&name).cloned()
}

// Function to get all recipes
#[ic_cdk::query]
fn get_all_recipes() -> Vec<Recipe> {
    let recipe_manager = RecipeManager::default();
    let all_recipes: Vec<Recipe> = recipe_manager.recipes.values().cloned().collect();
    all_recipes
}

ic_cdk::export_candid!();
