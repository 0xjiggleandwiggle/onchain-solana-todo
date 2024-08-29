use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    msg,
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{next_account_info, AccountInfo}
};

pub enum TodosInstructions {
    
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Todo {
    pub title: String,
    pub is_complete: bool
}

  
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TodosAccount {
    pub owner: Pubkey,
    pub todos: Vec<Todo>
}

pub fn todos_intruction(


) -> ProgramResult {

}


