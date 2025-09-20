# polygon-arbitrage-detector
A Rust-based bot that detects and simulates arbitrage opportunities between DEXs on the Polygon network.


# Polygon Arbitrage Opportunity Detector Bot 📈

A bot built in Rust that detects potential arbitrage opportunities on the Polygon PoS network. It monitors token prices on different Decentralized Exchanges (DEXs) and identifies profitable scenarios.

**Disclaimer:** This is a *detector and simulator* only. It does not execute trades.

---

## ## Architecture

The bot operates in a continuous loop:
1.  Loads configuration from a `.env` file.
2.  Establishes a WebSocket connection to a Polygon RPC node.
3.  Periodically fetches the price of a token pair (e.g., WETH/USDC) from multiple DEXs.
4.  Compares prices and calculates the simulated profit after accounting for a flat gas fee.
5.  If the profit exceeds a defined threshold, it prints the opportunity to the console.

---

## ## Technology Stack

* **Language:** Rust
* **Blockchain:** Polygon PoS
* **Core Libraries:**
    * `ethers-rs` for blockchain interaction.
    * `tokio` for the asynchronous runtime.
    * `dotenv` for configuration management.

---

## ## Setup and Configuration

1.  **Install Rust:** Follow the instructions at [rust-lang.org](https://www.rust-lang.org/).
2.  **Clone the repository:**
    ```bash
    git clone <Your-Repository-URL.git>
    cd polygon-arbitrage-detector
    ```
3.  **Create the configuration file:** Copy the `.env.example` file to a new file named `.env`.
    ```bash
    cp .env.example .env
    ```
4.  **Edit the `.env` file:** You must add your own Polygon WebSocket RPC URL.

---

## ## How to Run

Once the setup is complete, run the bot from the project's root directory with the following command:
```bash
cargo run
```
The bot will connect to the network and begin checking for opportunities.
