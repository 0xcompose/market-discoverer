# Service (eg. Coingecko, LayerZero, Stargate, OFT List):

-   Has Response Type, primarily generated with `quicktype`
-   Has Entity type, array of which that we store (in json rn)
-   Entity MUST HAVE ID, that we use to ensure Entity uniqueness and compare entity lists of known and recently fetched ones
-   Entity CAN have CHAIN_ID that can be used to cross-reference Entity of Service in entity list of other Services
-   Entity CAN have CAIP_ID that can be used to cross-reference Entity of Service in entity list of other Services and correctly handling non-EVM chains (Solana, Eclipse, Sui, Bitcoin, Aptos, etc.)

## Service Structure

service_name/

-   types.rs (can be auto-generated)
-   adapters.rs
-   fetch.rs // retrieval specific logic
-   diff.rs // (optional) custom implementation of diff for Service's Entity

Btw Service can incapsulate all processing logic, but just have default implementation for most of the functions
