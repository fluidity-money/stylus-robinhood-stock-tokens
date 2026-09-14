
# stylus-robinhood-stock-tokens

Helper functions to get info from Robinhood Stock Tokens.

Some results, including the `terms` function, the domain separator, EIP-712 fields, and the
price feed, are hardcoded. At the time of writing (14-09-26) this seems to be correct.

## Install

The default features use `stylus-sdk`:

```toml
[dependencies]
stylus-robinhood-stock-tokens = "0.1"
```

## Examples

Get hardcoded token metadata without making a contract call:

```rust
use stylus_robinhood_stock_tokens::StockToken;
use stylus_sdk::alloy_primitives::Address;

let token = StockToken::Aapl;
let address = Address::from(token.addr());
let name = token.name();
let decimals = token.decimals();
```

Read the AAPL oracle price and pause status from a Stylus contract:

```rust
extern crate alloc;

use alloc::vec::Vec;
use stylus_robinhood_stock_tokens::{paused, price, StockToken};
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    pub struct Example {}
}

#[public]
impl Example {
    pub fn aapl_price(&self) -> Result<U256, Vec<u8>> {
        price(self.vm(), Call::new(), StockToken::Aapl).map_err(|_| Vec::new())
    }

    pub fn aapl_paused(&self) -> Result<bool, Vec<u8>> {
        paused(self.vm(), Call::new(), StockToken::Aapl).map_err(|_| Vec::new())
    }
}
```
