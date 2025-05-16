use near_sdk::{
    borsh::BorshSerialize, env, json_types::U64, near, store::IterableMap, AccountId,
    BorshStorageKey, PanicOnDefault, StorageUsage,
};

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    StoreIterable,
    StoreLookup,
    CollectionsLookup,
}

#[near(serializers = [json])]
pub struct StorageUsageInfo {
    pub store_iterable_map_storage_usage: U64,
    pub store_lookup_map_storage_usage: U64,
    pub collections_lookup_map_storage_usage: U64,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct ContractData {
    pub store_iterable_map: IterableMap<AccountId, u128>,
    pub store_lookup_map: near_sdk::store::LookupMap<AccountId, u128>,
    pub collections_lookup_map: near_sdk::collections::LookupMap<AccountId, u128>,
    pub store_iterable_map_storage_usage: StorageUsage,
    pub store_lookup_map_storage_usage: StorageUsage,
    pub collections_lookup_map_storage_usage: StorageUsage,
}

#[near]
impl ContractData {
    #[init]
    pub fn new() -> Self {
        let mut c = Self {
            store_iterable_map: IterableMap::new(StorageKey::StoreIterable),
            store_lookup_map: near_sdk::store::LookupMap::new(StorageKey::StoreLookup),
            collections_lookup_map: near_sdk::collections::LookupMap::new(
                StorageKey::CollectionsLookup,
            ),
            store_iterable_map_storage_usage: 0,
            store_lookup_map_storage_usage: 0,
            collections_lookup_map_storage_usage: 0,
        };
        c.measure_storage();
        c
    }

    fn measure_storage(&mut self) {
        let tmp_account_id: AccountId = "0".repeat(64).parse().unwrap();
        {
            let initial_storage_usage = env::storage_usage();
            self.store_iterable_map.insert(tmp_account_id.clone(), 100);
            self.store_iterable_map_storage_usage = env::storage_usage() - initial_storage_usage;
        }
        {
            let initial_storage_usage = env::storage_usage();
            self.store_lookup_map.insert(tmp_account_id.clone(), 100);
            self.store_lookup_map_storage_usage = env::storage_usage() - initial_storage_usage;
        }
        {
            let initial_storage_usage = env::storage_usage();
            self.collections_lookup_map.insert(&tmp_account_id, &100);
            self.collections_lookup_map_storage_usage =
                env::storage_usage() - initial_storage_usage;
        }
    }

    pub fn check_storage_usage_info(&self) -> StorageUsageInfo {
        StorageUsageInfo {
            store_iterable_map_storage_usage: self.store_iterable_map_storage_usage.into(),
            store_lookup_map_storage_usage: self.store_lookup_map_storage_usage.into(),
            collections_lookup_map_storage_usage: self.collections_lookup_map_storage_usage.into(),
        }
    }
}
