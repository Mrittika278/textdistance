use criterion::{criterion_group, criterion_main, Criterion, black_box};
use textdistance_rs::{Distance, Similarity};
use textdistance_rs::edit::Levenshtein;
use textdistance_rs::edit::DamerauLevenshtein;
use textdistance_rs::edit::JaroWinkler;
use textdistance_rs::token::Jaccard;
use textdistance_rs::token::Sorensen;
use textdistance_rs::token::Cosine;
use textdistance_rs::sequence::LcsSeq;
use textdistance_rs::sequence::LcsStr;
use textdistance_rs::simple::Hamming;

fn bench_levenshtein(c: &mut Criterion) {
    let alg = Levenshtein;
    c.bench_function("levenshtein_short", |b| {
        b.iter(|| alg.distance(black_box("kitten"), black_box("sitting")))
    });
    c.bench_function("levenshtein_medium", |b| {
        b.iter(|| alg.distance(black_box("the quick brown fox"), black_box("the quick blue fox")))
    });
}

fn bench_damerau(c: &mut Criterion) {
    let alg = DamerauLevenshtein;
    c.bench_function("damerau_short", |b| {
        b.iter(|| alg.distance(black_box("ca"), black_box("ac")))
    });
    c.bench_function("damerau_medium", |b| {
        b.iter(|| alg.distance(black_box("the quick brown fox"), black_box("the quick blue fox")))
    });
}

fn bench_jaro_winkler(c: &mut Criterion) {
    let alg = JaroWinkler::new();
    c.bench_function("jaro_winkler_short", |b| {
        b.iter(|| alg.normalized_similarity(black_box("MARTHA"), black_box("MARHTA")))
    });
    c.bench_function("jaro_winkler_medium", |b| {
        b.iter(|| alg.normalized_similarity(black_box("the quick brown fox"), black_box("the quick blue fox")))
    });
}

fn bench_jaccard(c: &mut Criterion) {
    let alg = Jaccard;
    c.bench_function("jaccard_short", |b| {
        b.iter(|| alg.normalized_similarity(black_box("hello world"), black_box("hello there")))
    });
}

fn bench_lcsseq(c: &mut Criterion) {
    let alg = LcsSeq;
    c.bench_function("lcsseq_short", |b| {
        b.iter(|| alg.similarity_value(black_box("abcde"), black_box("ace")))
    });
}

fn bench_lcsstr(c: &mut Criterion) {
    let alg = LcsStr;
    c.bench_function("lcsstr_short", |b| {
        b.iter(|| alg.similarity_value(black_box("abcdef"), black_box("zabxycdef")))
    });
}

fn bench_hamming(c: &mut Criterion) {
    let alg = Hamming;
    c.bench_function("hamming_equal", |b| {
        b.iter(|| alg.distance(black_box("karolin"), black_box("kathrin")))
    });
}

fn bench_sorensen(c: &mut Criterion) {
    let alg = Sorensen;
    c.bench_function("sorensen_short", |b| {
        b.iter(|| alg.normalized_similarity(black_box("hello world"), black_box("hello there")))
    });
}

fn bench_cosine(c: &mut Criterion) {
    let alg = Cosine;
    c.bench_function("cosine_short", |b| {
        b.iter(|| alg.normalized_similarity(black_box("hello world"), black_box("hello there")))
    });
}

criterion_group!(benches, bench_levenshtein, bench_damerau, bench_jaro_winkler, bench_jaccard, bench_lcsseq, bench_lcsstr, bench_hamming, bench_sorensen, bench_cosine);
criterion_main!(benches);