use aho_corasick::AhoCorasick;
use azusa::Azusa;

#[test]
fn test() {
    let patterns = &["c", "😅", "é", "文"];
    let haystack = "c😅é文";

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((patterns[mat.pattern()], mat.start(), mat.end()));
    }

    let transformer = Azusa::new(haystack.into());
    let get = |idx: u32| {
        let item = matches.get(idx as usize).unwrap();
        println!("item: {:?}", item);
        transformer.utf8_to_utf16((item.1 as u32, item.2 as u32))
    };

    // c
    assert_eq!(get(0), (0, 1));
    // 😅
    assert_eq!(get(1), (1, 3));
    assert_eq!(transformer.utf8_to_utf16((1, 5)), (1, 3));
    // é
    assert_eq!(get(2), (3, 5));
    // 文
    assert_eq!(get(3), (5, 6));
}
