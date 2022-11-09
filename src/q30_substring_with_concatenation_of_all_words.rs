/*
 * @lc app=leetcode id=30 lang=rust
 *
 * [30] Substring with Concatenation of All Words
 */

// @lc code=start
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // println!("s: {:?}", s);
        // println!("words: {:?}", words);
        let mut result = Vec::new();
        // non-unique words (counting the repetitions)
        // let words = {
        //     let mut words = words;
        //     words.sort();
        //     words.dedup();
        //     words
        // };
        let word_len = words[0].len();
        let word_num = words.len();
        let concatenated_len = word_len * word_num;
        if s.len() < concatenated_len {
            return result;
        }
        let sorted_dedup_words = {
            let mut words = words.clone();
            words.sort();
            words.dedup();
            words
        };
        // give each word an id `i`(smallest possible) when sorted_dedup_words[i] == word
        // this helps dealing with repeated words
        let word_count = {
            let mut count = vec![0; sorted_dedup_words.len()];
            for word in words.clone() {
                let i = sorted_dedup_words.iter().position(|w| w == &word).unwrap();
                count[i] += 1;
            }
            count
        };
        // println!("word_count: {:?}", word_count);

        let mut pos_id_pairs_grouped = vec![];
        for id in 0..word_count.len() {
            let pos_id_pairs = {
                let mut v = vec![];
                for pos in 0..=s.len() - word_len {
                    let word = &s[pos..pos + word_len];
                    if word == sorted_dedup_words[id] {
                        v.push((pos, id));
                    }
                }
                v
            };

            // let pos_id_pairs: Vec<(usize, usize)> = s
            //     .match_indices(&sorted_dedup_words[id])
            //     .map(|(pos, _word)| (pos, id))
            //     .collect();
            // println!("pos_i_pairs: {:?}", pos_id_pairs);
            pos_id_pairs_grouped.push(pos_id_pairs);

            // let word_pos = pos_i_pairs
            //     .iter()
            //     .map(|(pos, _idx)| *pos)
            //     .collect::<Vec<_>>();
            // ith_word_pos.push(word_pos);
        }
        let pos_id_pairs_concat = pos_id_pairs_grouped.concat();
        // sort by pos
        // pos_id_pairs.sort_unstable_by_key(|pair| (pair).0);
        // pos_id_pairs.sort_by(|a, b| (*a).index(0) > (*b).index(0));
        // let all_word_pos = ith_word_pos.clone().concat();

        for offset in 0..word_len {
            // let indices_with_offset = all_word_pos
            //     .iter()
            //     .filter(|pos| **pos % word_len == offset)
            //     .collect::<Vec<&usize>>();
            let mut pos_id_pairs_with_offset = pos_id_pairs_concat
                .iter()
                .filter(|(pos, _idx)| pos % word_len == offset)
                .collect::<Vec<_>>();
            pos_id_pairs_with_offset.sort_by_key(|pair| pair.0);
            // pos_id_pairs_with_offset is sorted by pos (`pos_id_pairs.sort`)

            // let mut word_start_indices_in_s = indices_with_offset
            //     .iter()
            //     .map(|pos| {
            //         pos_idx_pairs
            //             .iter()
            //             .position(|pair| pair[0].0 == **pos)
            //             .unwrap()
            //     })
            //     .collect::<Vec<usize>>();
            // word_start_indices_in_s.sort(); // almost 0 execution time if sorted
            // println!("offset: {}", offset);
            // println!("pos_id_pairs_with_offset: {:?}", pos_id_pairs_with_offset);
            let possible_slices = pos_id_pairs_with_offset
                .windows(word_num)
                .filter(|slice| {
                    // println!("filter1 slice: {:?}", slice);
                    slice.last().unwrap().0 + word_len - slice.first().unwrap().0
                        == concatenated_len
                })
                .filter(|slice| {
                    // check slice contains all words
                    // println!("filter2 slice: {:?}", slice);
                    let mut word_appeared = vec![0; sorted_dedup_words.len()];
                    for (_, id) in slice.to_vec() {
                        word_appeared[*id] += 1;
                    }
                    // println!("word_appeared: {:?}", word_appeared);
                    word_appeared == word_count
                })
                .collect::<Vec<_>>();

            result.append(
                possible_slices
                    .iter()
                    .map(|slice| slice.first().unwrap().0 as i32)
                    .collect::<Vec<i32>>()
                    .as_mut(),
            );
        }

        result
    }
}
// @lc code=end

struct Solution;
pub fn main() {
    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );
    assert_eq!(
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        ),
        vec![]
    );
    assert_eq!(
        Solution::find_substring(
            "barfoofoobarthefoobarman".to_string(),
            vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
        ),
        vec![6, 9, 12]
    );
    assert_eq!(
        // 30/178
        Solution::find_substring("mississippi".to_string(), vec!["mississippis".to_string(),]),
        vec![]
    );
    assert_eq!(
        // 149/178
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "good".to_string()
            ]
        ),
        vec![8]
    );
    assert_eq!(
        // 161/178
        Solution::find_substring(
            "ababababab".to_string(),
            vec!["ababa".to_string(), "babab".to_string(),]
        ),
        vec![0]
    );

    assert_eq!(
        // 175/178
        Solution::find_substring(
            "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab".to_string(),
            vec!["ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "b≈a".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), "ab".to_string(), "ba".to_string(), ]
        ),
        vec![]
    );
}
