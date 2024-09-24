use std::collections::{HashMap, HashSet};

use divan::black_box;
use once_cell::sync::Lazy;
use phf::phf_set;
use radix_trie::Trie;
use trie_hard::TrieHard;

const OW_1984: &str = include_str!("../data/1984.txt");
const SUN_RISING: &str = include_str!("../data/sun-rising.txt");
const RANDOM: &str = include_str!("../data/random.txt");

// From https://github.com/pichillilorenzo/known-http-header-db/blob/main/src/db.json
const HEADERS: &str = include_str!("../data/headers.txt");
static HEADERS_REV: Lazy<Vec<String>> = Lazy::new(|| {
    HEADERS
        .lines()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
});

static HEADERS_PHF: phf::Set<&str> = phf_set! {
        "",
        "accept-additions",
        "accept-ch-lifetime",
        "accept-datetime",
        "accept-features",
        "accept-patch",
        "accept-ranges",
        "access-control-allow-credentials",
        "access-control-allow-methods",
        "access-control-expose-headers",
        "access-control-request-headers",
        "amp-cache-transform",
        "authentication-control",
        "authorization",
        "c-man",
        "c-pep",
        "cache-control",
        "cal-managed-id",
        "capsule-protocol",
        "cdn-loop",
        "cert-not-before",
        "close",
        "connection",
        "content-disposition",
        "content-encoding",
        "content-language",
        "content-location",
        "content-range",
        "content-security-policy",
        "content-style-type",
        "content-version",
        "cookie2",
        "cross-origin-embedder-policy-report-only",
        "cross-origin-opener-policy-report-only",
        "delta-base",
        "derived-from",
        "device-memory",
        "digest",
        "downlink",
        "early-data",
        "ediint-features",
        "expect",
        "expires",
        "feature-policy",
        "getprofile",
        "host",
        "if-modified-since",
        "if-range",
        "if-unmodified-since",
        "include-referred-token-binding-id",
        "keep-alive",
        "large-allocation",
        "last-modified",
        "location",
        "memento-datetime",
        "method-check",
        "mime-version",
        "odata-isolation",
        "odata-version",
        "optional-www-authenticate",
        "origin",
        "oscore",
        "overwrite",
        "permissions-policy",
        "ping-from",
        "position",
        "prefer",
        "priority",
        "protocol-query",
        "protocol",
        "proxy-authenticate",
        "proxy-authorization",
        "proxy-features",
        "proxy-status",
        "public-key-pins",
        "range",
        "referer",
        "referer[sic]",
        "referrer-policy",
        "repeatability-client-id",
        "repeatability-request-id",
        "replay-nonce",
        "retry-after",
        "schedule-reply",
        "sec-ch-ua-bitness",
        "sec-ch-ua-full-version-list",
        "sec-ch-ua-model",
        "sec-ch-ua-platform-version",
        "sec-ch-ua",
        "sec-fetch-mode",
        "sec-fetch-user",
        "sec-token-binding",
        "sec-websocket-extensions",
        "sec-websocket-protocol",
        "security-scheme",
        "server-timing",
        "set-cookie",
        "setprofile",
        "soapaction",
        "status",
        "strict-transport-security",
        "surrogate-capability",
        "timeout",
        "traceparent",
        "trailer",
        "upgrade-insecure-requests",
        "variant-vary",
        "want-digest",
        "x-att-deviceid",
        "x-content-security-policy",
        "x-correlation-id",
        "x-dns-prefetch-control",
        "x-forwarded-host",
        "x-frame-options",
        "x-powered-by",
        "x-request-id",
        "x-ua-compatible",
        "x-wap-profile",
        "x-xss-protection",
};

static SMALL_PHF: phf::Set<&str> = phf_set! {
    ";",
    "?",
    "",
    "\"All",
    "a",
    "age",
    "alchemy.",
    "alike,",
    "all",
    "All",
    "and",
    "And",
    "ants",
    "are",
    "art",
    "as",
    "Ask",
    "asks",
    "be",
    "Be",
    "beams",
    "bed",
    "blinded",
    "both",
    "BUSY",
    "but",
    "But",
    "call",
    "Call",
    "center",
    "chide",
    "clime,",
    "cloud",
    "compared",
    "contracted",
    "could",
    "country",
    "court-huntsmen",
    "curtains,",
    "days,",
    "do",
    "done",
    "dost",
    "duties",
    "ease,",
    "eclipse",
    "else",
    "everywhere",
    "eyes",
    "fool,",
    "for",
    "go",
    "Go",
    "half",
    "happy",
    "harvest",
    "have",
    "hear,",
    "her",
    "here",
    "honour's",
    "hours,",
    "I",
    "If",
    "in",
    "In",
    "Indias",
    "is,",
    "is",
    "king",
    "kings",
    "knows",
    "late",
    "Late",
    "lay.\"",
    "left'st",
    "lie",
    "long.",
    "Look,",
    "lose",
    "Love,",
    "lovers'",
    "me,",
    "me.",
    "mimic,",
    "mine",
    "months,",
    "motions",
    "Must",
    "no",
    "nor",
    "Nor",
    "not",
    "Nothing",
    "of",
    "offices",
    "old",
    "on",
    "one",
    "or",
    "pedantic",
    "play",
    "prentices,",
    "princes",
    "Princes",
    "rags",
    "reverend,",
    "ride,",
    "run",
    "Saucy",
    "saw'st",
    "school-boys",
    "season",
    "seasons",
    "shalt",
    "She's",
    "Shine",
    "shouldst",
    "sight",
    "since",
    "so",
    "sour",
    "sphere.",
    "spice",
    "states,",
    "strong",
    "Sun,",
    "tell",
    "th'",
    "that's",
    "that",
    "the",
    "them,",
    "them",
    "these",
    "thine,",
    "Thine",
    "think",
    "this,",
    "This",
    "those",
    "Thou,",
    "thou",
    "through",
    "Through",
    "thus,",
    "thus",
    "thy",
    "Thy",
    "time.",
    "to-morrow",
    "to",
    "To",
    "unruly",
    "us,",
    "us.",
    "us",
    "walls",
    "warm",
    "warming",
    "we,",
    "wealth",
    "where",
    "Whether",
    "which",
    "whom",
    "Why",
    "will",
    "windows,",
    "wink,",
    "with",
    "world,",
    "world's",
    "would",
    "wretch,",
    "yesterday,",
};

const PERCENT: &[i32] = &[100, 75, 50, 25, 10, 5, 2, 1];

fn main() {
    divan::main();
}

/* -------------------------------------------------------------------------- */
/*                                 BENCHMARKS                                 */
/* -------------------------------------------------------------------------- */

#[divan::bench(args = args())]
fn trie_get(bencher: divan::Bencher, input: &Input) {
    bencher
        .with_inputs(|| {
            let words = match input.size {
                Size::Header => get_header_text(),
                Size::Big => get_big_text(),
                Size::Small => get_small_text(),
            };
            let trie = make_trie(&words);
            (generate_samples(&words, input.percent), trie)
        })
        .bench_values(|(samples, trie): (Vec<&str>, TrieHard<'_, &str>)| {
            samples
                .iter()
                .filter_map(|w| trie.get(black_box(&w[..])))
                .count()
        });
}

#[divan::bench(args = args())]
fn radix_trie_get(bencher: divan::Bencher, input: &Input) {
    bencher
        .with_inputs(|| {
            let words = match input.size {
                Size::Header => get_header_text(),
                Size::Big => get_big_text(),
                Size::Small => get_small_text(),
            };
            let trie = make_radix_trie(&words);
            (generate_samples(&words, input.percent), trie)
        })
        .bench_values(|(samples, trie): (Vec<&str>, Trie<&str, usize>)| {
            samples
                .iter()
                .filter_map(|w| trie.get(black_box(&w[..])))
                .count()
        });
}

#[divan::bench(args = args())]
fn hashmap_get(bencher: divan::Bencher, input: &Input) {
    bencher
        .with_inputs(|| {
            let words = match input.size {
                Size::Header => get_header_text(),
                Size::Big => get_big_text(),
                Size::Small => get_small_text(),
            };
            let hashmap = make_hashmap(&words);
            (generate_samples(&words, input.percent), hashmap)
        })
        .bench_values(
            |(samples, hashmap): (Vec<&str>, HashMap<&str, &str>)| {
                samples
                    .iter()
                    .filter_map(|w| hashmap.get(black_box(&w[..])))
                    .count()
            },
        );
}

#[divan::bench(args = args_small())]
fn phf_get(bencher: divan::Bencher, input: &Input) {
    bencher
        .with_inputs(|| {
            let (words, phf) = match input.size {
                Size::Header => (get_header_text(), &HEADERS_PHF),
                Size::Small => (get_small_text(), &SMALL_PHF),
                Size::Big => unreachable!(),
            };
            (generate_samples(&words, input.percent), phf)
        })
        .bench_values(|(samples, phf): (Vec<&str>, &phf::Set<&str>)| {
            samples
                .iter()
                .filter_map(|w| phf.get_key(black_box(&w[..])))
                .count()
        });
}

#[divan::bench(args = &[Size::Big, Size::Small])]
fn trie_insert(bencher: divan::Bencher, size: &Size) {
    bencher
        .with_inputs(|| match size {
            Size::Header => get_header_text(),
            Size::Big => get_big_text(),
            Size::Small => get_small_text(),
        })
        .bench_values(|words: Vec<&str>| make_trie(black_box(&words)));
}

#[divan::bench(args = &[Size::Big, Size::Small])]
fn radix_trie_insert(bencher: divan::Bencher, size: &Size) {
    bencher
        .with_inputs(|| match size {
            Size::Header => get_header_text(),
            Size::Big => get_big_text(),
            Size::Small => get_small_text(),
        })
        .bench_values(|words: Vec<&str>| make_radix_trie(black_box(&words)));
}

#[divan::bench(args = &[Size::Big, Size::Small])]
fn hashmap_insert(bencher: divan::Bencher, size: &Size) {
    bencher
        .with_inputs(|| match size {
            Size::Header => get_header_text(),
            Size::Big => get_big_text(),
            Size::Small => get_small_text(),
        })
        .bench_values(|words: Vec<&str>| make_hashmap(black_box(&words)));
}

/* -------------------------------------------------------------------------- */
/*                                   INPUTS                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Eq)]
enum Size {
    Header,
    Big,
    Small,
}

struct Input {
    size: Size,
    percent: i32,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // divan sorts by lexicographic order, so we add padding to the percentage
        f.write_fmt(format_args!("{:?} - {:03}%", self.size, self.percent))
    }
}

fn args() -> impl Iterator<Item = Input> {
    PERCENT
        .iter()
        .map(|p| Input {
            size: Size::Header,
            percent: *p,
        })
        .chain(PERCENT.iter().map(|p| Input {
            size: Size::Big,
            percent: *p,
        }))
        .chain(PERCENT.iter().map(|p| Input {
            size: Size::Small,
            percent: *p,
        }))
}

fn args_small() -> impl Iterator<Item = Input> {
    PERCENT
        .iter()
        .map(|p| Input {
            size: Size::Header,
            percent: *p,
        })
        .chain(PERCENT.iter().map(|p| Input {
            size: Size::Small,
            percent: *p,
        }))
}

/* -------------------------------------------------------------------------- */
/*                                   HELPERS                                  */
/* -------------------------------------------------------------------------- */

fn get_big_text() -> Vec<&'static str> {
    OW_1984
        .split(|c: char| c.is_whitespace())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn get_small_text() -> Vec<&'static str> {
    SUN_RISING
        .split(|c: char| c.is_whitespace())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn get_header_text() -> Vec<&'static str> {
    HEADERS_REV.iter().map(|s| s.as_str()).collect()
}

fn get_random_text() -> Vec<&'static str> {
    RANDOM
        .split(|c: char| c.is_whitespace())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn make_trie<'a>(words: &[&'a str]) -> TrieHard<'a, &'a str> {
    words.iter().copied().collect()
}

fn make_hashmap<'a>(words: &[&'a str]) -> HashMap<&'a str, &'a str> {
    words.iter().map(|k| (*k, *k)).collect()
}

fn make_radix_trie<'a>(words: &[&'a str]) -> Trie<&'a str, usize> {
    let mut trie = Trie::new();
    for w in words {
        trie.insert(&w[..], w.len());
    }
    trie
}

fn generate_samples<'a>(hits: &[&'a str], hit_percent: i32) -> Vec<&'a str> {
    let roulette_inc = hit_percent as f64 / 100.;
    let mut roulette = 0.;

    let mut result = get_random_text().to_owned();
    let mut hit_iter = hits.iter().cycle().copied();

    for w in result.iter_mut() {
        roulette += roulette_inc;
        if roulette >= 1. {
            roulette -= 1.;
            *w = hit_iter.next().unwrap();
        }
    }

    result
}
