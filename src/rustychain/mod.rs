use blake2::{Blake2b, Digest};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Block {
    transactions: Vec<Transaction>,
    prev_hash: Option<String>,
    hash: Option<String>,
    nonce: u128,
}

impl Block {
    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some()
            && self
                .hash
                .as_ref()
                .unwrap()
                .eq(&byte_vector_to_string(&self.calculate_hash()))
        {
            true
        } else {
            false
        }
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();

        for v in self.transactions.iter() {
            hasher.update(v.calculate_hash());
        }

        hasher.update(format!("{:?}", (self.prev_hash, self.nonce)));

        Vec::from(hasher.finalize().as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, Account>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        let is_genesis = self.blocks.len() == 0;
        if !block.verify_own_hash() {
            return Err("Block hash breach");
        }
    }
}

trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return a account given it's id if is available (mutable)
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Will return a account given it's id if is available
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Will add a new account
    fn create_account(&mut self, id: String, account_type: AccountType)
        -> Result<(), &'static str>;
}

#[derive(Debug, Clone)]
pub struct Transaction {
    nonce: u128,
    from: String,
    created_at: SystemTime,
    pub(crate) record: TransactionData,
    signature: Option<String>,
}

impl Transaction {
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();
        let transaction_as_string = format!(
            "{:?}",
            (&self.created_at, &self.record, &self.from, &self.nonce)
        );

        hasher.update(&transaction_as_string);
        return Vec::from(hasher.finalize().as_ref());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionData {
    CreateUserAccount(String),

    ChangeStoreValue { key: String, value: String },

    TransferTokens { to: String, amoutn: u128 },

    CreateTokens { receiver: String, amount: u128 },
}

#[derive(Debug, Clone)]
pub struct Account {
    store: HashMap<String, String>,
    account_type: AccountType,
    tokens: u128,
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            tokens: 0,
            account_type: account_type,
            store: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
    },
}
