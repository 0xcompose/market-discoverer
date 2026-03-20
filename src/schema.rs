// @generated automatically by Diesel CLI.

diesel::table! {
    ethereum_list_chains (id) {
        id -> Integer,
        chain_id -> Integer,
        chain_name -> Text,
        chain_type -> Text,
        chain_status -> Text,
        chain_layer -> Text,
        chain_stack -> Text,
        chain_native_currency -> Text,
        chain_native_currency_address -> Text,
        chain_native_currency_decimals -> Integer,
        chain_native_currency_symbol -> Text,
        chain_native_currency_name -> Text,
    }
}
