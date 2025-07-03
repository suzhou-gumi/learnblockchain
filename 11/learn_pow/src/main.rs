use sha2::{Digest, Sha256};
use std::time::Instant;

/// 计算满足指定前导零数量的SHA256哈希
fn pow(nickname: &str, zero_prefix: usize) {
    let prefix = "0".repeat(zero_prefix);
    let mut nonce: u64 = 0;
    let start = Instant::now();

    loop {
        // 拼接内容
        let content = format!("{}{}", nickname, nonce);
        // 计算SHA256
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        let hash_hex = hex::encode(&result);

        // 检查前缀
        if hash_hex.starts_with(&prefix) {
            let elapsed = start.elapsed();
            println!(
                "满足前导{}个0的哈希:\n花费时间: {:?}\n内容: {}\nHash值: {}",
                zero_prefix, elapsed, content, hash_hex
            );
            break;
        }
        nonce += 1;
    }
}

fn main() {
    let nickname = "gumi";
    // 4个0
    pow(nickname, 4);
    // 5个0
    pow(nickname, 5);
}
