Data Science Student Registry dAPP

A decentralized application built on the Stellar Network (Soroban) to manage and verify Data Science student records. This project was developed as part of a blockchain workshop.

Features

Register Student: Save student details (Name, NIM, Expertise) onto the Stellar Testnet.

View Registry: Fetch the complete list of registered Data Science students.

Delete Record: Remove a student record using their unique ID.

Blockchain Powered: All data is stored immutably on the Stellar Testnet.

Project Structure

contracts/: Soroban smart contract written in Rust.

frontend/: React-based user interface to interact with the contract.

target/: Build artifacts (WASM files).

Smart Contract Details

Network: Stellar Testnet

Contract ID: [DEPLOYED_CONTRACT_ID_WILL_APPEAR_HERE]

Admin/Source Account: brama (Example ID)

How to Deploy to Testnet

Create Wallet:

stellar keys generate brama --network testnet --fund


Build Contract:

stellar contract build


Deploy:

stellar contract deploy --source-account brama --network testnet --wasm target/wasm32-unknown-unknown/release/student_registry.wasm


Interaction

To fetch students via CLI:

stellar contract invoke --id [YOUR_CONTRACT_ID] --source-account brama --network testnet -- get_students

