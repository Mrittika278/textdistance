use std::io::{self, Write};

use textdistance_rs::{Distance, Similarity};

use textdistance_rs::simple::{Identity, Prefix, Postfix, Hamming};
use textdistance_rs::edit::{Levenshtein, DamerauLevenshtein, Jaro, JaroWinkler};
use textdistance_rs::token::{Jaccard, Sorensen, Overlap, Cosine, Tversky, Bag};
use textdistance_rs::sequence::{LcsSeq, LcsStr, RatcliffObershelp};
use textdistance_rs::phonetic::{Mra, Editex};
use textdistance_rs::compression::{
    RLENCD,
    BWTRLENCD,
    SqrtNCD,
    EntropyNCD,
    ZLIBNCD,
    NCD,
};

/// Helper function to prompt the user and read a clean input string
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim trailing newline / carriage return characters
    input.trim().to_string()
}

fn main() {
    println!();
    println!("==============================================================");
    println!("              TEXTDISTANCE-RS INTERACTIVE DEMO");
    println!("==============================================================");
    println!();

    // Get input strings from the user
    let str1 = get_user_input("Enter first string  : ");
    let str2 = get_user_input("Enter second string : ");

    println!();
    println!("Comparing: \"{}\" <-> \"{}\"", str1, str2);
    println!();

    // =========================================================
    // SIMPLE
    // =========================================================
    println!("==================== SIMPLE ====================");

    let identity = Identity;
    println!(
        "Identity                 : {}",
        identity.similarity_value(&str1, &str2)
    );

    let prefix = Prefix;
    println!(
        "Prefix Similarity        : {}",
        prefix.similarity_value(&str1, &str2)
    );

    let postfix = Postfix;
    println!(
        "Postfix Similarity       : {}",
        postfix.similarity_value(&str1, &str2)
    );

    let hamming = Hamming;
    if str1.chars().count() == str2.chars().count() {
        println!(
            "Hamming Distance         : {}",
            hamming.distance(&str1, &str2)
        );
    } else {
        println!("Hamming Distance         : N/A (Strings must be equal length)");
    }

    println!();

    // =========================================================
    // EDIT
    // =========================================================
    println!("===================== EDIT =====================");

    let lev = Levenshtein;
    println!(
        "Levenshtein              : {}",
        lev.distance(&str1, &str2)
    );

    let dam = DamerauLevenshtein;
    println!(
        "Damerau-Levenshtein      : {}",
        dam.distance(&str1, &str2)
    );

    let jaro = Jaro::new();
    println!(
        "Jaro Similarity          : {:.3}",
        jaro.normalized_similarity(&str1, &str2)
    );

    let jw = JaroWinkler::new();
    println!(
        "Jaro-Winkler Similarity  : {:.3}",
        jw.normalized_similarity(&str1, &str2)
    );

    println!();

    // =========================================================
    // TOKEN
    // =========================================================
    println!("==================== TOKEN =====================");

    let jac = Jaccard;
    println!(
        "Jaccard                  : {:.3}",
        jac.normalized_similarity(&str1, &str2)
    );

    let sor = Sorensen;
    println!(
        "Sorensen                 : {:.3}",
        sor.normalized_similarity(&str1, &str2)
    );

    let overlap = Overlap;
    println!(
        "Overlap                  : {:.3}",
        overlap.normalized_similarity(&str1, &str2)
    );

    let cosine = Cosine;
    println!(
        "Cosine                   : {:.3}",
        cosine.normalized_similarity(&str1, &str2)
    );

    let tv = Tversky::new();
    println!(
        "Tversky                  : {:.3}",
        tv.normalized_similarity(&str1, &str2)
    );

    let bag = Bag;
    println!(
        "Bag Distance             : {}",
        bag.distance(&str1, &str2)
    );

    println!();

    // =========================================================
    // SEQUENCE
    // =========================================================
    println!("=================== SEQUENCE ===================");

    let lcs = LcsSeq;
    println!(
        "LCS Sequence             : {}",
        lcs.similarity_value(&str1, &str2)
    );

    let lcss = LcsStr;
    println!(
        "LCS Substring            : {}",
        lcss.similarity_value(&str1, &str2)
    );

    let rat = RatcliffObershelp;
    println!(
        "Ratcliff-Obershelp       : {:.3}",
        rat.normalized_similarity(&str1, &str2)
    );

    println!();

    // =========================================================
    // PHONETIC
    // =========================================================
    println!("=================== PHONETIC ===================");

    let mra = Mra;
    println!(
        "MRA Similarity           : {}",
        mra.similarity_value(&str1, &str2)
    );

    let editex = Editex::new();
    println!(
        "Editex Distance          : {}",
        editex.distance(&str1, &str2)
    );

    println!();

    // =========================================================
    // COMPRESSION
    // =========================================================
    println!("================= COMPRESSION ==================");

    let rle = RLENCD;
    println!(
        "RLENCD                   : {:.3}",
        rle.distance(&[&str1, &str2])
    );

    let bwt = BWTRLENCD::default();
    println!(
        "BWTRLENCD                : {:.3}",
        bwt.distance(&[&str1, &str2])
    );

    let sqrt = SqrtNCD;
    println!(
        "SqrtNCD                  : {:.3}",
        sqrt.distance(&str1, &str2)
    );

    let entropy = EntropyNCD::default();
    println!(
        "EntropyNCD               : {:.3}",
        entropy.distance(&str1, &str2)
    );

    let zlib = ZLIBNCD;
    println!(
        "ZLIBNCD                  : {:.3}",
        zlib.distance(&str1, &str2)
    );

    println!();

    println!("==============================================================");
    println!("Executed all distance and similarity algorithms successfully.");
    println!("==============================================================");
}