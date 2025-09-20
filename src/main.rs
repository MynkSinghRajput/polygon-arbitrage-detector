use ethers::abi::parse_abi;
use ethers::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

struct Config {
    rpc_url: String,
    weth_address: Address,
    usdc_address: Address,
    dex_routers: Vec<Address>,
    trade_amount: U256,
    min_profit_threshold: f64,
    simulated_gas_cost: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = load_config()?;
    let provider = Provider::<Ws>::connect(&config.rpc_url).await?;
    let client = Arc::new(provider);
    println!("✅ Successfully connected to Polygon RPC. Bot is running...");

    loop {
        let trade_path = vec![config.weth_address, config.usdc_address];
        let mut prices = Vec::new();

        for &router in &config.dex_routers {
            let amount_out = get_amounts_out(
                router,
                config.trade_amount,
                trade_path.clone(),
                client.clone(),
            )
            .await;
            
            if let Ok(amount) = amount_out {
                let readable_amount = ethers::utils::format_units(amount, "mwei")?.parse::<f64>()?;
                prices.push((router, readable_amount));
            }
        }

        // --- ARBITRAGE DETECTION LOGIC ---
        if prices.len() == 2 {
            let (dex_a_addr, price_a) = prices[0];
            let (dex_b_addr, price_b) = prices[1];

            // Scenario 1: Buy on A (cheaper), Sell on B (pricier)
            if price_b > price_a {
                let profit = (price_b - price_a) - config.simulated_gas_cost;
                if profit > config.min_profit_threshold {
                    println!("\n💰 Arbitrage Opportunity Found! 💰");
                    println!("  - Buy 1 WETH on DEX A ({:?}) for {:.2} USDC", dex_a_addr, price_a);
                    println!("  - Sell 1 WETH on DEX B ({:?}) for {:.2} USDC", dex_b_addr, price_b);
                    println!("  - Simulated Profit: {:.2} USDC\n", profit);
                }
            }

            // Scenario 2: Buy on B (cheaper), Sell on A (pricier)
            if price_a > price_b {
                let profit = (price_a - price_b) - config.simulated_gas_cost;
                if profit > config.min_profit_threshold {
                    println!("\n💰 Arbitrage Opportunity Found! 💰");
                    println!("  - Buy 1 WETH on DEX B ({:?}) for {:.2} USDC", dex_b_addr, price_b);
                    println!("  - Sell 1 WETH on DEX A ({:?}) for {:.2} USDC", dex_a_addr, price_a);
                    println!("  - Simulated Profit: {:.2} USDC\n", profit);
                }
            }
        }
        
        sleep(Duration::from_secs(30)).await;
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let rpc_url = env::var("POLYGON_RPC_URL")?;
    let weth_address: Address = env::var("WETH_ADDRESS")?.parse()?;
    let usdc_address: Address = env::var("USDC_ADDRESS")?.parse()?;
    let quickswap_router: Address = env::var("QUICKSWAP_ROUTER")?.parse()?;
    let sushiswap_router: Address = env::var("SUSHISWAP_ROUTER")?.parse()?;
    let trade_amount_str = env::var("TRADE_AMOUNT_WETH")?;
    let trade_amount = ethers::utils::parse_ether(&trade_amount_str)?;
    let min_profit_threshold = env::var("MIN_PROFIT_THRESHOLD")?.parse::<f64>()?;
    let simulated_gas_cost = env::var("SIMULATED_GAS_COST_USD")?.parse::<f64>()?;
    Ok(Config {
        rpc_url,
        weth_address,
        usdc_address,
        dex_routers: vec![quickswap_router, sushiswap_router],
        trade_amount,
        min_profit_threshold,
        simulated_gas_cost,
    })
}

async fn get_amounts_out(
    router_address: Address,
    amount_in: U256,
    path: Vec<Address>,
    client: Arc<Provider<Ws>>,
) -> Result<U256, Box<dyn std::error::Error>> {
    let abi = parse_abi(&["function getAmountsOut(uint256,address[]) returns (uint256[])"])?;
    let contract = Contract::new(router_address, abi, client);
    let amounts: Vec<U256> = contract
        .method("getAmountsOut", (amount_in, path))?
        .call()
        .await?;
    Ok(*amounts.last().unwrap_or(&U256::zero()))
}