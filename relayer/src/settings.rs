use config::{ConfigError, Config, Environment};

#[derive(Debug, Deserialize)]
pub struct Relayer {
    pub n1_rpc_url: String,
    pub n1_chain_id: String,
    pub n2_rpc_url: String,
    pub n2_chain_id: String,
    pub n1_gas_price: String,
    pub n1_gas_limit: String,
    pub n2_gas_price: String,
    pub n2_gas_limit: String,
    pub dapp_mnemonic: String,
    pub dapp_address: String,
    pub amb_mnemonic: String,
    pub amb_address: String,
    pub counter_n1: String,
    pub counter_n2: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub r: Relayer,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Add settings from the environment
        s.merge(Environment::with_prefix("r"))?;

        // Relayer settings
        s.set("r.n1_rpc_url", s.get::<String>("n1_rpc_url")
            .unwrap_or_else(|_| "n1_rpc_url is missing".into()))?;
        s.set("r.n1_chain_id", s.get::<String>("n1_chain_id")
            .unwrap_or_else(|_| "n1_chain_id is missing".into()))?;
        s.set("r.n2_rpc_url", s.get::<String>("n2_rpc_url")
            .unwrap_or_else(|_| "n2_rpc_url is missing".into()))?;
        s.set("r.n2_chain_id", s.get::<String>("n2_chain_id")
            .unwrap_or_else(|_| "n2_chain_id is missing".into()))?;
        s.set("r.n1_gas_price", s.get::<String>("n1_gas_price")
            .unwrap_or_else(|_| "n1_gas_price is missing".into()))?;
        s.set("r.n1_gas_limit", s.get::<String>("n1_gas_limit")
            .unwrap_or_else(|_| "n1_gas_limit is missing".into()))?;
        s.set("r.n2_gas_price", s.get::<String>("n2_gas_price")
            .unwrap_or_else(|_| "n2_gas_price is missing".into()))?;
        s.set("r.n2_gas_limit", s.get::<String>("n2_gas_limit")
            .unwrap_or_else(|_| "n2_gas_limit is missing".into()))?;

        s.set("r.dapp_mnemonic", s.get::<String>("dapp_mnemonic")
            .unwrap_or_else(|_| "dapp_mnemonic is missing".into()))?;
        s.set("r.dapp_address", s.get::<String>("dapp_address")
            .unwrap_or_else(|_| "dapp_address is missing".into()))?;
        s.set("r.amb_mnemonic", s.get::<String>("amb_mnemonic")
            .unwrap_or_else(|_| "amb_mnemonic is missing".into()))?;
        s.set("r.amb_address", s.get::<String>("amb_address")
            .unwrap_or_else(|_| "amb_address is missing".into()))?;

        s.set("r.counter_n1", s.get::<String>("counter_n1")
            .unwrap_or_else(|_| "counter_n1 is missing".into()))?;
        s.set("r.counter_n2", s.get::<String>("counter_n2")
            .unwrap_or_else(|_| "counter_n2 is missing".into()))?;

        // Deserialize / freeze configuration
        s.try_into()
    }
}