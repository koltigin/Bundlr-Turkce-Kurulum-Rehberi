table! {
    bundle (id) {
        id -> Nullable<Text>,
        owner_address -> Nullable<Text>,
        block_height -> BigInt,
    }
}

table! {
    leaders (address) {
        address -> Nullable<Text>,
    }
}

table! {
    transactions (id) {
        id -> Nullable<Text>,
        epoch -> BigInt,
        block_promised -> BigInt,
        block_actual -> Nullable<BigInt>,
        signature -> Binary,
        validated -> Bool,
        bundle_id -> Nullable<Text>,
        sent_to_leader -> Bool,
    }
}

table! {
    validators (address) {
        address -> Nullable<Text>,
        url -> Nullable<Text>,
    }
}

joinable!(leaders -> validators (address));
joinable!(transactions -> bundle (bundle_id));

allow_tables_to_appear_in_same_query!(bundle, leaders, transactions, validators,);
