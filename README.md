Script to calculate electricity bill based on consumption and electricity prices.

Expects input from .csv file exported from Elering (https://estfeed.elering.ee/metering-data-consumption)


# Build

`cargo build`

# Run

`cargo run input.csv`

 or from pre-compiled binary:  

 `./target/debug/electricity_market_price_bill input.csv`
