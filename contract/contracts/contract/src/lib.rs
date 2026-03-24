#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, String, Vec, Map, Address
};

#[derive(Clone)]
#[contracttype]
pub struct Ad {
    pub id: u32,
    pub owner: Address,
    pub title: String,
    pub content: String,
}

#[contracttype]
pub enum DataKey {
    Ads,        // Map<u32, Ad>
    AdIds,      // Vec<u32>
}

#[contract]
pub struct AdContract;

#[contractimpl]
impl AdContract {

    // 🧾 Create a new ad
    pub fn create_ad(env: Env, id: u32, owner: Address, title: String, content: String) {
        
        // Get ads map
        let mut ads: Map<u32, Ad> = env
            .storage()
            .instance()
            .get(&DataKey::Ads)
            .unwrap_or(Map::new(&env));

        // Get ID list
        let mut ids: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::AdIds)
            .unwrap_or(Vec::new(&env));

        // Create ad
        let ad = Ad {
            id,
            owner,
            title,
            content,
        };

        // Store ad
        ads.set(id, ad);

        // Store ID if not already present
        if !ids.contains(id) {
            ids.push_back(id);
        }

        // Save back to storage
        env.storage().instance().set(&DataKey::Ads, &ads);
        env.storage().instance().set(&DataKey::AdIds, &ids);
    }

    // 🔍 Get a single ad
    pub fn get_ad(env: Env, id: u32) -> Option<Ad> {
        let ads: Map<u32, Ad> = env
            .storage()
            .instance()
            .get(&DataKey::Ads)
            .unwrap_or(Map::new(&env));

        ads.get(id)
    }

    // 📦 Get all ads
    pub fn get_all_ads(env: Env) -> Vec<Ad> {
        let ads: Map<u32, Ad> = env
            .storage()
            .instance()
            .get(&DataKey::Ads)
            .unwrap_or(Map::new(&env));

        let ids: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::AdIds)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for id in ids.iter() {
            if let Some(ad) = ads.get(id) {
                result.push_back(ad);
            }
        }

        result
    }
}