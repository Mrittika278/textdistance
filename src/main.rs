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

fn main() {
    println!();
    println!("==============================================================");
    println!("              TEXTDISTANCE-RS DEMONSTRATION");
    println!("==============================================================");
    println!();

    println!("Sample Inputs:");
    println!("\"kitten\"  <->  \"sitting\"");
    println!();

    // =========================================================
    // SIMPLE
    // =========================================================

    println!("==================== SIMPLE ====================");

    let identity = Identity;
    println!(
        "Identity                 : {}",
        identity.similarity_value("hello", "hello")
    );

    let prefix = Prefix;
    println!(
        "Prefix Similarity        : {}",
        prefix.similarity_value("hello", "help")
    );

    let postfix = Postfix;
    println!(
        "Postfix Similarity       : {}",
        postfix.similarity_value("hello", "jello")
    );

    let hamming = Hamming;
    println!(
        "Hamming Distance         : {}",
        hamming.distance("karolin", "kathrin")
    );

    println!();

    // =========================================================
    // EDIT
    // =========================================================

    println!("===================== EDIT =====================");

    let lev = Levenshtein;
    println!(
        "Levenshtein              : {}",
        lev.distance("kitten", "sitting")
    );

    let dam = DamerauLevenshtein;
    println!(
        "Damerau-Levenshtein      : {}",
        dam.distance("ca", "ac")
    );

    let jaro = Jaro::new();
    println!(
        "Jaro Similarity          : {:.3}",
        jaro.normalized_similarity("MARTHA", "MARHTA")
    );

    let jw = JaroWinkler::new();
    println!(
        "Jaro-Winkler Similarity  : {:.3}",
        jw.normalized_similarity("MARTHA", "MARHTA")
    );

    println!();

    // =========================================================
    // TOKEN
    // =========================================================

    println!("==================== TOKEN =====================");

    let jac = Jaccard;
    println!(
        "Jaccard                  : {:.3}",
        jac.normalized_similarity("abc", "abd")
    );

    let sor = Sorensen;
    println!(
        "Sorensen                 : {:.3}",
        sor.normalized_similarity("abc", "abd")
    );

    let overlap = Overlap;
    println!(
        "Overlap                  : {:.3}",
        overlap.normalized_similarity("abc", "abd")
    );

    let cosine = Cosine;
    println!(
        "Cosine                   : {:.3}",
        cosine.normalized_similarity("abc", "abd")
    );

    let tv = Tversky::new();
    println!(
        "Tversky                  : {:.3}",
        tv.normalized_similarity("abc", "abd")
    );

    let bag = Bag;
    println!(
        "Bag Distance             : {}",
        bag.distance("abc", "abd")
    );

    println!();

    // =========================================================
    // SEQUENCE
    // =========================================================

    println!("=================== SEQUENCE ===================");

    let lcs = LcsSeq;
    println!(
        "LCS Sequence             : {}",
        lcs.similarity_value("abcde", "ace")
    );

    let lcss = LcsStr;
    println!(
        "LCS Substring            : {}",
        lcss.similarity_value("abcdef", "zabxycdef")
    );

    let rat = RatcliffObershelp;
    println!(
        "Ratcliff-Obershelp       : {:.3}",
        rat.normalized_similarity("abcde", "abfde")
    );

    println!();

    // =========================================================
    // PHONETIC
    // =========================================================

    println!("=================== PHONETIC ===================");

    let mra = Mra;
    println!(
        "MRA Similarity           : {}",
        mra.similarity_value("Smith", "Smyth")
    );

    let editex = Editex::new();
    println!(
        "Editex Distance          : {}",
        editex.distance("cat", "kat")
    );

    println!();

    // =========================================================
    // COMPRESSION
    // =========================================================

    println!("================= COMPRESSION ==================");

    let rle = RLENCD;
    println!(
        "RLENCD                   : {:.3}",
        rle.distance(&["banana", "banana"])
    );

    let bwt = BWTRLENCD::default();
    println!(
        "BWTRLENCD                : {:.3}",
        bwt.distance(&["banana", "banana"])
    );

    let sqrt = SqrtNCD;
    println!(
        "SqrtNCD                  : {:.3}",
        sqrt.distance("banana", "bandana")
    );

    let entropy = EntropyNCD::default();
    println!(
        "EntropyNCD               : {:.3}",
        entropy.distance("banana", "bandana")
    );

    let zlib = ZLIBNCD;
    println!(
        "ZLIBNCD                  : {:.3}",
        zlib.distance("banana", "bandana")
    );

    println!();

    println!("==============================================================");
    println!("Implemented Algorithms : 24");
    println!("Language               : Rust 🦀");
    println!("Project                : textdistance-rs");
    println!("Status                 : All algorithms executed successfully.");
    println!("==============================================================");
}