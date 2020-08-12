### Token Information Quote

A commandline tool to quote token price and other related information. It's based on CoinGecko API v3.

### examples

```bash
# checker server status
rust-gecko ping

# quote bitcoin and ethereum (token list should be comma separated) price:
rust-gecko simple -i bitcoin,ethereum -v usd

# quote bitcoin price against ethereum with other options
rust-gecko simple -i bitcoin -v eth -o include_market_cap=true:include_24hr_vol=true

# quote ethereum information
rust-gecko coins -i ethereum


# quote ethereum information with other options, warning of long output
rust-gecko coins -i ethereum -o localization=true:tickers=true
```
