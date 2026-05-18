#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Todos,
    Counter,
    Points(Address),
}

const REWARD_PER_TASK: u32 = 10;

#[contract]
pub struct TodoRewardContract;

#[contractimpl]
impl TodoRewardContract {
    pub fn create_task(env: Env, user: Address, title: String) -> u64 {
        user.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(&env));

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let todo = Todo {
            id: counter,
            owner: user,
            title,
            completed: false,
        };

        todos.push_back(todo);

        env.storage().instance().set(&DataKey::Todos, &todos);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    pub fn get_tasks(env: Env) -> Vec<Todo> {
        env.storage()
            .instance()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_my_tasks(env: Env, user: Address) -> Vec<Todo> {
        let todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(&env));

        let mut my_tasks = Vec::new(&env);

        for i in 0..todos.len() {
            let todo = todos.get(i).unwrap();

            if todo.owner == user {
                my_tasks.push_back(todo);
            }
        }

        my_tasks
    }

    pub fn complete_task(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();

            if todo.id == id {
                if todo.owner != user {
                    return String::from_str(&env, "Task bukan milik user ini");
                }

                if todo.completed {
                    return String::from_str(&env, "Task sudah selesai");
                }

                todo.completed = true;
                todos.set(i, todo);

                let current_points: u32 = env
                    .storage()
                    .instance()
                    .get(&DataKey::Points(user.clone()))
                    .unwrap_or(0);

                let new_points = current_points + REWARD_PER_TASK;

                env.storage()
                    .instance()
                    .set(&DataKey::Points(user), &new_points);

                env.storage().instance().set(&DataKey::Todos, &todos);

                return String::from_str(&env, "Task selesai, reward ditambahkan");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    pub fn delete_task(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let todo = todos.get(i).unwrap();

            if todo.id == id {
                if todo.owner != user {
                    return String::from_str(&env, "Task bukan milik user ini");
                }

                todos.remove(i);
                env.storage().instance().set(&DataKey::Todos, &todos);

                return String::from_str(&env, "Task berhasil dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    pub fn get_points(env: Env, user: Address) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Points(user))
            .unwrap_or(0)
    }
}

mod test;