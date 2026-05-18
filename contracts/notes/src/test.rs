#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

#[test]
fn test_create_task() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TodoRewardContract, ());
    let client = TodoRewardContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let title = String::from_str(&env, "Belajar Soroban");

    let task_id = client.create_task(&user, &title);

    assert_eq!(task_id, 1);

    let tasks = client.get_tasks();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks.get(0).unwrap().id, 1);
    assert_eq!(tasks.get(0).unwrap().owner, user);
    assert_eq!(tasks.get(0).unwrap().title, title);
    assert_eq!(tasks.get(0).unwrap().completed, false);
}

#[test]
fn test_complete_task_and_get_reward() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TodoRewardContract, ());
    let client = TodoRewardContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let title = String::from_str(&env, "Selesaikan task pertama");

    let task_id = client.create_task(&user, &title);

    let result = client.complete_task(&user, &task_id);

    assert_eq!(
        result,
        String::from_str(&env, "Task selesai, reward ditambahkan")
    );

    let tasks = client.get_tasks();
    let task = tasks.get(0).unwrap();

    assert_eq!(task.completed, true);

    let points = client.get_points(&user);

    assert_eq!(points, 10);
}

#[test]
fn test_complete_task_twice_should_not_add_reward_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TodoRewardContract, ());
    let client = TodoRewardContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let title = String::from_str(&env, "Task tidak boleh double reward");

    let task_id = client.create_task(&user, &title);

    let first_result = client.complete_task(&user, &task_id);
    let second_result = client.complete_task(&user, &task_id);

    assert_eq!(
        first_result,
        String::from_str(&env, "Task selesai, reward ditambahkan")
    );

    assert_eq!(
        second_result,
        String::from_str(&env, "Task sudah selesai")
    );

    let points = client.get_points(&user);

    assert_eq!(points, 10);
}

#[test]
fn test_delete_task() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TodoRewardContract, ());
    let client = TodoRewardContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let title = String::from_str(&env, "Task yang akan dihapus");

    let task_id = client.create_task(&user, &title);

    let result = client.delete_task(&user, &task_id);

    assert_eq!(result, String::from_str(&env, "Task berhasil dihapus"));

    let tasks = client.get_tasks();

    assert_eq!(tasks.len(), 0);
}

#[test]
fn test_get_my_tasks() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TodoRewardContract, ());
    let client = TodoRewardContractClient::new(&env, &contract_id);

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    let title_1 = String::from_str(&env, "Task user 1");
    let title_2 = String::from_str(&env, "Task user 2");

    client.create_task(&user_1, &title_1);
    client.create_task(&user_2, &title_2);

    let user_1_tasks = client.get_my_tasks(&user_1);
    let user_2_tasks = client.get_my_tasks(&user_2);

    assert_eq!(user_1_tasks.len(), 1);
    assert_eq!(user_2_tasks.len(), 1);

    assert_eq!(user_1_tasks.get(0).unwrap().title, title_1);
    assert_eq!(user_2_tasks.get(0).unwrap().title, title_2);
}