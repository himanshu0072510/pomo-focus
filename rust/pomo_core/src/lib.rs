use wasm_bindgen::prelude::*;
use serde::Deserialize;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::{Write, Read};
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, OsRng, Key, Nonce}};
use aes_gcm::aead::rand_core::RngCore;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};

#[wasm_bindgen]
pub fn compute_focus_score(total_poms: u32, focus_minutes: u32, streak: u32, tasks_done: u32) -> f64 {
    let raw = total_poms as f64 * 12.0
            + focus_minutes as f64 * 0.4
            + streak as f64 * 6.0
            + tasks_done as f64 * 4.0;
    (raw / (1.0 + (total_poms as f64 * 0.05))).min(1000.0)
}

#[wasm_bindgen]
pub fn best_hours(hour_counts: &[u32]) -> Vec<usize> {
    let mut indexed: Vec<(usize,u32)> = hour_counts.iter().enumerate().map(|(i,&c)|(i,c)).collect();
    indexed.sort_by(|a,b| b.1.cmp(&a.1));
    indexed.into_iter().take(5).filter(|(_,c)| *c>0).map(|(i,_)|i).collect()
}

#[wasm_bindgen]
pub fn compress_text(input: &str) -> String {
    let mut enc = GzEncoder::new(Vec::new(), Compression::default());
    enc.write_all(input.as_bytes()).ok();
    let bytes = enc.finish().unwrap_or_default();
    B64.encode(bytes)
}

#[wasm_bindgen]
pub fn decompress_text(input: &str) -> String {
    let bytes = match B64.decode(input) { Ok(b)=>b, Err(_)=>return String::new() };
    let mut dec = GzDecoder::new(&bytes[..]);
    let mut out = String::new();
    dec.read_to_string(&mut out).ok();
    out
}

fn key_from_password(pw: &str) -> [u8;32] {
    let mut h = Sha256::new();
    h.update(pw.as_bytes());
    h.finalize().into()
}

#[wasm_bindgen]
pub fn encrypt_text(plaintext: &str, password: &str) -> String {
    let key_bytes = key_from_password(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8;12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
    match cipher.encrypt(nonce, plaintext.as_bytes()) {
        Ok(ct) => {
            let mut out = Vec::with_capacity(12+ct.len());
            out.extend_from_slice(&nonce_bytes);
            out.extend_from_slice(&ct);
            B64.encode(out)
        }
        Err(_) => String::new()
    }
}

#[wasm_bindgen]
pub fn decrypt_text(ciphertext_b64: &str, password: &str) -> String {
    let data = match B64.decode(ciphertext_b64) { Ok(d)=>d, Err(_)=>return String::new() };
    if data.len() < 13 { return String::new(); }
    let (nonce_bytes, ct) = data.split_at(12);
    let key_bytes = key_from_password(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce_bytes);
    match cipher.decrypt(nonce, ct) {
        Ok(pt) => String::from_utf8(pt).unwrap_or_default(),
        Err(_) => String::new()
    }
}

#[wasm_bindgen]
pub fn extract_keywords(text: &str) -> Vec<String> {
    let stop = ["the","a","an","and","or","to","of","in","on","for","with","is","it","this","that","i","you","we","my","your"];
    let mut words: Vec<String> = text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3 && !stop.contains(w))
        .map(|w| w.to_string())
        .collect();
    words.sort();
    words.dedup();
    words.into_iter().take(8).collect()
}

#[derive(Deserialize)]
struct StatEntry { poms: Option<u32>, focus_min: Option<u32> }

#[wasm_bindgen]
pub fn productivity_insights(stats_json: &str) -> String {
    // stats_json: object date-> {poms, focus_min}
    let parsed: serde_json::Value = match serde_json::from_str(stats_json) { Ok(v)=>v, Err(_)=>return "{}".to_string() };
    let mut total_poms = 0u32;
    let mut total_min = 0u32;
    let mut days = 0u32;
    if let Some(obj) = parsed.as_object() {
        for (_k,v) in obj {
            total_poms += v.get("poms").and_then(|x|x.as_u64()).unwrap_or(0) as u32;
            total_min += v.get("focusMin").or(v.get("focus_min")).and_then(|x|x.as_u64()).unwrap_or(0) as u32;
            days += 1;
        }
    }
    let avg_poms = if days>0 { total_poms as f64 / days as f64 } else { 0.0 };
    let avg_min = if days>0 { total_min as f64 / days as f64 } else { 0.0 };
    let score = compute_focus_score(total_poms, total_min, 0, 0);
    format!(r#"{{"total_poms":{},"total_minutes":{},"days":{},"avg_poms":{:.2},"avg_minutes":{:.2},"score":{:.2}}}"#, total_poms, total_min, days, avg_poms, avg_min, score)
}

#[wasm_bindgen]
pub fn challenge_progress(current: u32, target: u32) -> f64 {
    if target == 0 { return 0.0; }
    (current as f64 / target as f64 * 100.0).min(100.0)
}

#[wasm_bindgen]
pub fn dft_magnitude(samples: &[f32]) -> Vec<f32> {
    let n = samples.len();
    if n == 0 { return vec![]; }
    let half = n/2;
    let mut out = vec![0.0f32; half];
    for k in 0..half {
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        for (i,&x) in samples.iter().enumerate() {
            let angle = -2.0 * std::f32::consts::PI * (k as f32) * (i as f32) / (n as f32);
            re += x * angle.cos();
            im += x * angle.sin();
        }
        out[k] = (re*re + im*im).sqrt();
    }
    out
}

#[wasm_bindgen]
pub fn generate_forest_seed(count: u32) -> u32 {
    // simple deterministic-ish seed mixer
    let mut x = count.wrapping_mul(2654435761);
    x ^= x >> 16;
    x = x.wrapping_mul(2246822519);
    x
}
