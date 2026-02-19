use std::env;
use std::fs;

fn main() {
    let input_path = env::args().nth(1).expect("Provide input file");
    let content = fs::read_to_string(input_path).expect("Failed to read file");

    let cleaned = remove_block_comments(&content);

    let mut variants = vec![];

    for line in cleaned.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let line = line.trim_end_matches(',').trim_end_matches(';');
        let name = match line.find('(') {
            Some(idx) => &line[..idx],
            None => line,
        }
            .trim();

        if !name.is_empty() {
            variants.push(name.to_string());
        }
    }

    // --- Generate Rust enum ---
    println!("pub enum Material {{");
    for var in &variants {
        println!("    {},", to_rust_variant(var));
    }
    println!("}}\n");

    // --- Generate to_java ---
    println!("impl<'caller> Material {{");
    println!("    pub fn to_java(&self, env: &mut Env<'caller>) -> jni::errors::Result<jni::objects::JObject<'caller>> {{");
    println!("        let class = env.find_class(jni_str!(\"org/bukkit/Material\"))?;");
    println!("        match self {{");

    for var in &variants {
        let rust_name = to_rust_variant(var);
        println!(
            "            Material::{} => env.get_static_field(class, jni_str!(\"{}\"), jni_sig!(\"Lorg/bukkit/Material;\"))?.l(),",
            rust_name, var
        );
    }

    println!("        }}");
    println!("    }}");
    println!("}}");
}

fn remove_block_comments(input: &str) -> String {
    let mut result = String::new();
    let mut in_comment = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if in_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            }
        } else if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
        } else {
            result.push(c);
        }
    }

    result
}

fn to_rust_variant(java_name: &str) -> String {
    java_name
        .split('_')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect()
}
