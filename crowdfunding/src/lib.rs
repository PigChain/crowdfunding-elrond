#![no_std]

imports!();

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {

    #[init]
    fn init(&self) {
    }
}