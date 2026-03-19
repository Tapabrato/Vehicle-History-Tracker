#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Vehicle {
    pub vin: String,           // Unique Vehicle ID
    pub owner: String,         // Owner info
    pub history: Vec<String>,  // History records
}

#[contract]
pub struct VehicleHistoryTracker;

#[contractimpl]
impl VehicleHistoryTracker {

    // Register a new vehicle
    pub fn register_vehicle(env: Env, vin: String, owner: String) {
        let history = Vec::new(&env);

        let vehicle = Vehicle {
            vin: vin.clone(),
            owner,
            history,
        };

        env.storage().instance().set(&vin, &vehicle);
    }

    // Add history record
    pub fn add_record(env: Env, vin: String, record: String) {
        let mut vehicle: Vehicle = env
            .storage()
            .instance()
            .get(&vin)
            .expect("Vehicle not found");

        vehicle.history.push_back(record);

        env.storage().instance().set(&vin, &vehicle);
    }

    // Get full vehicle details
    pub fn get_vehicle(env: Env, vin: String) -> Vehicle {
        env.storage()
            .instance()
            .get(&vin)
            .expect("Vehicle not found")
    }

    // Get only history
    pub fn get_vehicle_history(env: Env, vin: String) -> Vec<String> {
        let vehicle: Vehicle = env
            .storage()
            .instance()
            .get(&vin)
            .expect("Vehicle not found");

        vehicle.history
    }
}