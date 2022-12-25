/*
 * Model
 */

 #[derive(Debug, sqlx::FromRow)]
 pub struct Price {
    date: String,
    value: i32
}