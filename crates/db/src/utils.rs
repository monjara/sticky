use ulid::Ulid;

pub fn gen_id() -> String {
    Ulid::new().to_string()
}
