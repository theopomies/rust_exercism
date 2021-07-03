use std::collections::HashMap;
use std::iter::once;

pub fn tally(match_results: &str) -> String {
    let mut tally = HashMap::new();
    match_results
        .split('\n')
        .map(|l| l.split(';').collect())
        .for_each(|v: Vec<_>| {
            match v[..] {
                [home, ext, "win"] => {
                    tally.entry(home).or_insert((0, 0, 0)).0 += 1;
                    tally.entry(ext).or_insert((0, 0, 0)).2 += 1;
                }
                [home, ext, "draw"] => {
                    tally.entry(home).or_insert((0, 0, 0)).1 += 1;
                    tally.entry(ext).or_insert((0, 0, 0)).1 += 1;
                }
                [home, ext, "loss"] => {
                    tally.entry(home).or_insert((0, 0, 0)).2 += 1;
                    tally.entry(ext).or_insert((0, 0, 0)).0 += 1;
                }
                _ => (),
            };
        });
    let tally = {
        let mut sorted = tally.iter().collect::<Vec<_>>();
        sorted.sort_by(|&(&t1, &(w1, d1, _)), &(&t2, &(w2, d2, _))| {
            match (w2 * 3 + d2).cmp(&(w1 * 3 + d1)) {
                std::cmp::Ordering::Equal => t1.cmp(t2),
                v => v,
            }
        });
        sorted.into_iter().map(|(name, (win, draw, loss))| {
            format!(
                "{:<30} |{:3} |{:3} |{:3} |{:3} |{:3}",
                name,
                win + draw + loss,
                win,
                draw,
                loss,
                win * 3 + draw
            )
        })
    };
    once("Team                           | MP |  W |  D |  L |  P".to_owned())
        .chain(tally)
        .collect::<Vec<_>>()
        .join("\n")
}
