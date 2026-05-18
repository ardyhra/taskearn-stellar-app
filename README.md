
# TaskEarn

## Project Description

TaskEarn is a simple blockchain-based Todo application built with Soroban smart contracts on the Stellar network. The project allows users to create tasks, complete tasks, delete tasks, and earn reward points whenever they successfully complete a task.

Unlike a traditional Todo application, TaskEarn uses wallet-based ownership. Each task is linked to a Stellar wallet address, and only the task owner can complete or delete their own tasks. This makes the application a practical introduction to decentralized identity, smart contract storage, and user authorization on Soroban.

## Smart Contract ID

```txt
CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z
````

## Project Vision

The vision of TaskEarn is to demonstrate how simple productivity applications can be enhanced with Web3 features such as ownership, authorization, transparency, and rewards.

In the future, this project can evolve into a decentralized productivity platform where users are rewarded for completing personal goals, building habits, joining challenges, or participating in community-based task systems.

## Key Features

* Create a new todo task
* Store tasks on-chain using Soroban contract storage
* Assign each task to a wallet address
* Complete a task and earn reward points
* Prevent users from completing the same task twice
* Delete tasks owned by the user
* View all tasks stored in the contract
* View tasks belonging to a specific user
* View total reward points owned by a user
* Use wallet authorization with `require_auth()`

## Contract Functions

### `create_task`

Creates a new task for a user.

```rust
create_task(user: Address, title: String) -> u64
```

Returns the ID of the newly created task.

### `get_tasks`

Returns all tasks stored in the contract.

```rust
get_tasks() -> Vec<Todo>
```

### `get_my_tasks`

Returns all tasks owned by a specific wallet address.

```rust
get_my_tasks(user: Address) -> Vec<Todo>
```

### `complete_task`

Marks a task as completed and adds reward points to the task owner.

```rust
complete_task(user: Address, id: u64) -> String
```

A user can only complete their own task. A completed task cannot be rewarded twice.

### `delete_task`

Deletes a task owned by the user.

```rust
delete_task(user: Address, id: u64) -> String
```

Only the owner of the task can delete it.

### `get_points`

Returns the total reward points of a user.

```rust
get_points(user: Address) -> u32
```

## Data Structure

The main data structure used in this project is `Todo`.

```rust
pub struct Todo {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub completed: bool,
}
```

Each todo contains:

* `id`: unique task ID
* `owner`: wallet address of the task creator
* `title`: task title
* `completed`: task completion status

## Reward System

Each completed task gives the user a fixed reward.

```rust
const REWARD_PER_TASK: u32 = 10;
```

When a user completes a task, the contract adds 10 points to that user's total points.

The reward is stored on-chain as points, not as a token. This keeps the project simple and beginner-friendly while still demonstrating the reward mechanism.

## Tech Stack

* Rust
* Soroban SDK
* Stellar Smart Contract Platform
* Stellar CLI
* Soroban Studio

Optional frontend stack:

* React
* Next.js
* TypeScript
* Freighter Wallet
* Stellar JavaScript SDK

## How It Works

1. A user connects with a Stellar wallet address.
2. The user creates a new task.
3. The task is stored in the smart contract.
4. The task owner completes the task.
5. The smart contract checks authorization using `require_auth()`.
6. If the task belongs to the user and has not been completed before, the task status is updated.
7. The user receives reward points.
8. The user can view their tasks and total points.

## Authorization

TaskEarn uses Soroban's address authorization system.

```rust
user.require_auth();
```

This ensures that only the actual wallet owner can create, complete, or delete tasks on behalf of their address.

## Example Use Cases

* Personal productivity tracker
* Habit tracking with points
* Learning platform with task rewards
* Community challenge system
* Web3 onboarding project
* Beginner portfolio project for Soroban development

## Future Improvements

* Add deadline support for tasks
* Add task categories
* Add daily streak rewards
* Add leaderboard system
* Add NFT achievement badges
* Convert reward points into a real token
* Add admin-created challenges
* Add frontend integration with Freighter Wallet
* Add user profile system
* Add event logs for task creation and completion

## Getting Started

### Build the Contract

```bash
stellar contract build
```

### Deploy the Contract

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/todo.wasm \
  --source alice \
  --network testnet
```

### Invoke the Contract

Create a task:

```bash
stellar contract invoke \
  --id CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z \
  --source alice \
  --network testnet \
  -- \
  create_task \
  --user YOUR_STELLAR_ADDRESS \
  --title "Learn Soroban"
```

Get all tasks:

```bash
stellar contract invoke \
  --id CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z \
  --source alice \
  --network testnet \
  -- \
  get_tasks
```

Complete a task:

```bash
stellar contract invoke \
  --id CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z \
  --source alice \
  --network testnet \
  -- \
  complete_task \
  --user YOUR_STELLAR_ADDRESS \
  --id 1
```

Get user points:

```bash
stellar contract invoke \
  --id CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z \
  --source alice \
  --network testnet \
  -- \
  get_points \
  --user YOUR_STELLAR_ADDRESS
```

Delete a task:

```bash
stellar contract invoke \
  --id CBR4VVH77AOMJEIS52ZEH7ZP6UHSIRYFERUEHCUZX636VFHUDP6CYK6Z \
  --source alice \
  --network testnet \
  -- \
  delete_task \
  --user YOUR_STELLAR_ADDRESS \
  --id 1
```

## Project Status

This project is currently a simple smart contract prototype. It is designed for learning, testing, and portfolio development.

## License

This project is open-source and can be used for learning, experimentation, and further development.

````

Catatan kecil: pastikan nama file WASM di bagian deploy sesuai dengan nama package contract kamu. Kalau file hasil build bukan `todo.wasm`, ubah bagian ini:

```bash
target/wasm32v1-none/release/todo.wasm
````

sesuai nama file yang muncul di folder `target/wasm32v1-none/release/`.
