use crate::helpers::PartyId;
use itertools::Itertools;
use reed_solomon::{Decoder, Encoder};

pub(super) fn rs_encode_stripes(msg: &[u8], m: usize, k: usize) -> Vec<Vec<u8>> {
    // todo: use library that supports reed solomon with ecc_len > 256
    assert!(k <= usize::from(u8::MAX));

    let ecc_len = m - k; // we add m - k error codes
    let enc = Encoder::new(ecc_len);

    // Encode msg in chunks of k bytes to have len(encode(block)) = len(block) + ecc_len == m
    let msg = pad_pkcs7(msg, k as u8); // pad message

    let chunks: Vec<Vec<u8>> = msg.chunks(k).map(|c| enc.encode(c).to_vec()).collect();

    // Output an array of m stripes with len(m) / k chunks each
    (0..m)
        .map(|i| chunks.iter().map(|c| c[i]).collect())
        .collect()
}

pub(super) fn rs_decode_stripes(
    stripes: &[&Vec<u8>],
    missing: &[usize],
    m: usize,
    k: usize,
) -> Option<Vec<u8>> {
    assert!(k <= usize::from(u8::MAX));
    if stripes.is_empty() {
        None?
    }

    let missing: Vec<u8> = missing.iter().map(|i| *i as u8).collect();

    let ecc_len = m - k; // we add m - k error codes
    let dec = Decoder::new(ecc_len);

    // Convert stripes into chunks, i.e., chunks[i] = stripes[0][i] || stripes[1][i] || ... || stripes[n-1][i]
    let chunks =
        (0..stripes[0].len()).map(|i| stripes.iter().map(|row| row[i]).collect::<Vec<u8>>());

    // Decode msg as decode(chunks[0]) || decode(chunks[1]) || ... || decode(chunks[n-1])
    let msg = chunks
        .map(|c| dec.correct(&c, Some(&missing)).map(|v| v.data().to_vec()))
        .flatten_ok()
        .collect::<Result<Vec<u8>, _>>()
        .ok()?;

    // unpad message
    let msg = unpad_pkcs7(&msg)?;
    Some(msg.to_vec())
}

/// Fill the missing stripes and output the missing indices.
/// This is required because the positioning must be maintained for error correction.
pub(super) fn fill_missing_stripes<'a>(
    stripes: &[&'a Vec<u8>],
    indices: &[PartyId],
    n: usize,
) -> (Vec<&'a Vec<u8>>, Vec<usize>) {
    if stripes.is_empty() {
        return (vec![], vec![]);
    }

    let empty_stripe = stripes[0];
    let mut pos = 0;
    let mut complete_stripes = vec![];
    let mut missing_stripes_pos = vec![];
    for i in PartyId::iter_all(n) {
        if indices.get(pos) == Some(&i) {
            complete_stripes.push(stripes[pos]);
            pos += 1;
        } else {
            complete_stripes.push(empty_stripe);
            missing_stripes_pos.push(i.as_index());
        }
    }

    (complete_stripes, missing_stripes_pos)
}

/// Pkcs7 padding
fn pad_pkcs7(m: &[u8], block_size: u8) -> Vec<u8> {
    let block_usize = usize::from(block_size);
    let padlen = (block_usize - (m.len() % block_usize)) as u8; // safe cast, a - b, a >= b and a, b are u8
    if padlen == 0 {
        [m, vec![block_size; block_usize].as_slice()].concat()
    } else {
        [m, vec![padlen; usize::from(padlen)].as_slice()].concat()
    }
}

/// Remove a pkcs7 padding
fn unpad_pkcs7(m: &[u8]) -> Option<&[u8]> {
    let n = m.len();
    if n == 0 {
        return Some(m);
    }

    let padlen = usize::from(m[n - 1]);
    if padlen == 0 || padlen > n {
        None?
    } else {
        Some(&m[..n - padlen])
    }
}
