// 274. H-Index

pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut paper_counts = vec![0; citations.len() + 1];

    let mut h_index = citations.len();

    for c in citations {
        paper_counts[c.min(h_index as i32) as usize] += 1;
    }

    let mut papers = paper_counts[h_index];

    while papers < h_index {
        h_index -= 1;
        papers += paper_counts[h_index];
    }

    h_index as i32
}
