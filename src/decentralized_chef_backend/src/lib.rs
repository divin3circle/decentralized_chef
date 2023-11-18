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
    // Check for duplicate recipe name
    if let Some(existing_recipe) = RecipeManager::load().recipes.get(&name) {
        // Raise error or return appropriate message indicating conflict
        error!("Recipe with name '{}' already exists: {:?}", name, existing_recipe);
        return;
    }

    // Create and initialize new recipe object
    let recipe = Recipe {
        name: name.clone(),
        category: category.clone(),
        ingredients: ingredients.clone(),
        instructions: instructions.clone(),
    };

    // Access RecipeManager instance safely
    let mut recipe_manager = RecipeManager::load();

    // Insert new recipe into the recipes map
    recipe_manager.recipes.insert(name.clone(), recipe.clone());

    // Update categories map
    if !recipe_manager.categories.contains_key(&category) {
        recipe_manager.categories.insert(category.clone(), vec![name.clone()]);
    } else {
        recipe_manager.categories.get_mut(&category).unwrap().push(name.clone());
    }

    // Save the updated RecipeManager instance
    recipe_manager.save();
}

// Function to search for recipes by category
#[ic_cdk::query]
fn search_by_category(category: String) -> Vec<Recipe> {
    // Sanitize input to prevent malicious characters or expressions
    let sanitized_category = sanitize_input(category);

    // Access RecipeManager instance safely
    let recipe_manager = RecipeManager::load();

    // Retrieve recipes based on the sanitized category
    if let Some(recipe_names) = recipe_manager.categories.get(&sanitized_category) {
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
    // Sanitize input to prevent malicious characters or expressions
    let sanitized_name = sanitize_input(name);

    // Access RecipeManager instance safely
    let recipe_manager = RecipeManager::load();

    // Retrieve recipe based on the sanitized name
    recipe_manager.recipes.get(&sanitized_name).cloned()
}

// Function to get all recipes
#[ic_cdk::query]
fn get_all_recipes() -> Vec<Recipe> {
    // Access RecipeManager instance safely
    let recipe_manager = RecipeManager::load();

    // Efficient retrieval of all recipes
    let all_recipes: Vec<Recipe> = recipe_manager.recipes.values().cloned().collect();
    all_recipes
}

ic_cdk::export_candid!();
