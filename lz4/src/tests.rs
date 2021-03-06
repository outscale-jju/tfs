//! Tests.

use std::str;

use {decompress, compress};

/// Test that the compressed string decompresses to the original string.
fn inverse(s: &str) {
    let compressed = compress(s.as_bytes());
    println!("Compressed '{}' into {:?}", s, compressed);
    let decompressed = decompress(&compressed).unwrap();
    println!("Decompressed it into {:?}", str::from_utf8(&decompressed).unwrap());
    assert_eq!(decompressed, s.as_bytes());
}

#[test]
fn shakespear() {
    inverse("to live or not to live");
    inverse("Love is a wonderful terrible thing");
}

#[test]
fn totally_not_antifa_propaganda() {
    inverse("The only good fascist is a dead fascist.");
    inverse("bash the fash");
    inverse("Dead fascists can't vote.");
    inverse("Good night, white pride.");
    inverse("Some say fascism started with gas chambers. I say that's where it ends.");
}

#[test]
fn not_compressible() {
    inverse("as6yhol.;jrew5tyuikbfewedfyjltre22459ba");
}

#[test]
fn short() {
    inverse("ahhd");
}

#[test]
fn empty_string() {
    inverse("");
}

#[test]
fn compression_works() {
    let s = "micah (Micah Cohen, politics editor): Clinton’s lead has shrunk to a hair above 4 percentage points in our polls-only model, down from about 7 points two weeks ago. So we find ourselves in an odd position where Clinton still holds a clear lead, but it’s shrinking by the day. I’ve been getting questions from Clinton supporters wondering how panicked they should be, and while we advise everyone of all political stripes to always remain calm, let’s try to answer that question today. How safe is Clinton’s lead/how panicked should Democrats be? As tacky as it is to cite your own tweet, I’m going to do it anyway — here’s a handy scale: natesilver: It’s uncertain, in part, because of the risk of a popular vote-Electoral College split. And, in part, because there are various reasons to think polling error could be high this year, such as the number of undecided voters. You can see those forces at play in the recent tightening. Clinton hasn’t really declined very much in these latest polls. But she was at only 46 percent in national polls, and that left a little bit of wiggle room for Trump.";

    inverse(s);

    assert!(compress(s.as_bytes()).len() < s.len());
}
