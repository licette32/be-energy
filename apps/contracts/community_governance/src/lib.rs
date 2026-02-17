#![no_std]

//! # Community Governance Contract
//!
//! Sistema de gobernanza comunitaria para toma de decisiones del Hive.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub proposer: Address,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    ProposalCount,
    Proposal(u32),
}

#[contract]
pub struct CommunityGovernance;

#[contractimpl]
impl CommunityGovernance {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    pub fn create_proposal(env: Env, proposer: Address, title: String) -> u32 {
        proposer.require_auth();
        let count: u32 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        let id = count + 1;
        
        let proposal = Proposal {
            id,
            title,
            proposer,
            votes_for: 0,
            votes_against: 0,
        };
        
        env.storage().instance().set(&DataKey::Proposal(id), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &id);
        id
    }

    pub fn get_proposal_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0)
    }
}
