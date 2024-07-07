fn main() {
    dotenvy::from_path("../../.env").expect("Failed to load .env file");
}
