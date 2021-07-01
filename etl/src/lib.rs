use std::collections::BTreeMap;

// h.iter()
//     .flat_map(|(&v, vec)| vec.iter().map(move |l| (l.to_ascii_lowercase(), v)))
//     .collect()
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut map, (&v, vec)| {
        vec.iter().for_each(|l| {
            map.insert(l.to_ascii_lowercase(), v);
        });
        map
    })
}
