use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!(); 

#[near_bindgen]
#[derive(Default, BoshSerialize, BoshDeserialize)]
pub struct Counter {
    val: i8, 
} 

#[near_bindgen] 
impl Counter {
    pub fn get_num(&self) -> i8 {
        return self.val
    }

    pub fn increament(&mut self) {
        self.val += 1; 
        let log_message = format!("The value has been increased to {}", self.val); 
        env::log(log_message.as_bytes()); 
        after_counter_change(); 
    } 

    pub fn decreament(&mut self) {
        self.val -= 1; 
        let log_message = format!("The value has been decreased to {}", self.val); 
        env::log(log_message().as_bytes()); 
        after_counter_change(); 
    } 
    
    pub fn reset(&mut self) {
        self.val = 0; 
        env::log(b"Reset to zero"); 
    }
}

fn after_counter_change() {
    env::log("Make sure you don't overflow my friend".as_bytes())
}

// Unit tests 
#[cfg(test)]
mod tests {
    use super::*; 
    use near_sdk::MockedBlockchain; 
    use near_sdk::{testing_env, VMcontext}; 

    fn get_context(input: Vec, is_view: bool) -> VMcontext {
        VMcontext {
            current_account_id: "alice.testnet".to_string(), 
            signer_account_id: "robert.testnet".to_string(), 
            signer_account_pk: vec![0,1,2], 
            predecessor_account_id: "jane.testnet".to_string(), 
            input, 
            block_index: 0,  
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18), 
            random_seed: vec![0,1,2], 
            is_view, 
            output_data_receivers: vec![],
            epoch_height: 19,   
        } 
    }

    #[test] 
    fn increament() {
        let context = get_context(vec![], false); 
        testing_env!(context); 

        let mut contract = Counter {val: 0}; 
        contract.increament();
        println!("Value after increament: {}", contract.get_num()); 
        assert_eq!(1, contract.get_num())
    }

    #[test] 
    fn decreament() {
        let context = get_context(vec![], false); 
        testing_env!(context); 

        let mut contract = Counter { val: 0}; 
        contract.decreament(); 
        println!("Value after decreament: {}", contract.get_num());  
        assert_eq!(-1, contract.get_num()); 
    }

    #[test] 
    fn increment_and_reset() {
        let context = get_context(vec![], false); 
        testing_env!(context); 

        let mut contract = Counter { val: 0}; 
        contract.increment(); 
        contract.reset(); 
        println!("Value after reset: {}", contract.get_num()); 
        assert_eq!(0, contract.get_num())
    }
}