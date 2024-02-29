use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY};

pub fn create_hash(input: &str) -> String {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    context.update(input.as_bytes());
    let digest = context.finish();
    let hash_bytes = digest.as_ref();
    let hash_string = hex::encode(hash_bytes);
    hash_string[0..6].to_string()
}