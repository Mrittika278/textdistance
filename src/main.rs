use textdistance_rs::{Distance, Similarity};

use textdistance_rs::simple::{Identity, Prefix, Postfix, Hamming};
use textdistance_rs::edit::{Levenshtein, DamerauLevenshtein, Jaro, JaroWinkler};
use textdistance_rs::token::{Jaccard, Sorensen, Overlap, Cosine, Tversky, Bag};
use textdistance_rs::sequence::{LcsSeq, LcsStr, RatcliffObershelp};
use textdistance_rs::phonetic::{Mra, Editex};

fn main() {
    println!("\n============================================================");
    println!("              TEXTDISTANCE-RS DEMONSTRATION");
    println!("============================================================\n");

    println!("Input 1 : \"kitten\"");
    println!("Input 2 : \"sitting\"\n");

    println!("---------------- SIMPLE ALGORITHMS ----------------");

    let identity = Identity;
    println!("Identity                : {}", identity.similarity_value("hello", "hello"));

    let prefix = Prefix;
    println!("Prefix Similarity       : {}", prefix.similarity_value("hello", "help"));

    let postfix = Postfix;
    println!("Postfix Similarity      : {}", postfix.similarity_value("hello", "jello"));

    let hamming = Hamming;
    println!("Hamming Distance        : {}", hamming.distance("karolin", "kathrin"));

    println!();

    println!("---------------- EDIT DISTANCE --------------------");

    let lev = Levenshtein;
    println!("Levenshtein Distance    : {}", lev.distance("kitten", "sitting"));

    let dam = DamerauLevenshtein;
    println!("Damerau Distance        : {}", dam.distance("ca", "ac"));

    let jaro = Jaro::new();
    println!("Jaro Similarity         : {:.3}", jaro.normalized_similarity("MARTHA", "MARHTA"));

    let jw = JaroWinkler::new();
    println!("Jaro-Winkler Similarity : {:.3}", jw.normalized_similarity("MARTHA", "MARHTA"));

    println!();

    println!("---------------- TOKEN BASED ----------------------");

    let jac = Jaccard;
    println!("Jaccard Similarity      : {:.3}", jac.normalized_similarity("abc", "abd"));

    let sor = Sorensen;
    println!("Sorensen Similarity     : {:.3}", sor.normalized_similarity("abc", "abd"));

    let over = Overlap;
    println!("Overlap Similarity      : {:.3}", over.normalized_similarity("abc", "abd"));

    let cos = Cosine;
    println!("Cosine Similarity       : {:.3}", cos.normalized_similarity("abc", "abd"));

    let tv = Tversky::new();
    println!("Tversky Similarity      : {:.3}", tv.normalized_similarity("abc", "abd"));

    let bag = Bag;
    println!("Bag Distance            : {}", bag.distance("abc", "abd"));

    println!();

    println!("---------------- SEQUENCE BASED -------------------");

    let lcs = LcsSeq;
    println!("LCS Sequence            : {}", lcs.similarity_value("abcde", "ace"));

    let lcss = LcsStr;
    println!("LCS Substring           : {}", lcss.similarity_value("abcdef", "zabxycdef"));

    let rat = RatcliffObershelp;
    println!("Ratcliff-Obershelp      : {:.3}", rat.normalized_similarity("abcde", "abfde"));

    println!();

    println!("---------------- PHONETIC -------------------------");

    let mra = Mra;
    println!("MRA Similarity          : {}", mra.similarity_value("Smith", "Smyth"));

    let editex = Editex::new();
    println!("Editex Distance         : {}", editex.distance("cat", "kat"));

    println!("\n============================================================");
    println!("Implemented Algorithms : 19");
    println!("Language               : Rust 🦀");
    println!("Status                 : All algorithms executed successfully.");
    println!("============================================================");
}