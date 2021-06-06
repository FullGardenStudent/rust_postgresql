table! {
    playerdata (id) {
        id -> Int4,
        xcord -> Float8,
        ycord -> Float8,
        zcord -> Float8,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    user_table (id) {
        id -> Int4,
        x_cord -> Nullable<Float4>,
        y_cord -> Nullable<Float4>,
        z_cord -> Nullable<Float4>,
    }
}

table! {
    userdata (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        rating -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    playerdata,
    user,
    user_table,
    userdata,
);
