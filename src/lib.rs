// helpers
pub mod helper;

// List Node
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> ListNode {
        let mut result = ListNode::new(0);
        let mut tail_ref = &mut result;
        for i in vec {
            let n = Box::new(ListNode::new(i));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
        }
        result
    }
    pub fn from(vec: Vec<i32>) -> ListNode {
        ListNode::from_vec(vec)
    }
}

// solutions
pub mod q1_two_sum;
pub mod q2_add_two_numbers;
pub mod q3_longest_substring_without_repeating_characters;
pub mod q4_median_of_two_sorted_arrays;
pub mod q5_longest_palindromic_substring;
pub mod q6_zigzag_conversion;
pub mod q7_reverse_integer;
pub mod q8_string_to_integer_atoi;
pub mod q9_palindrome_number;
pub mod q10_regular_expression_matching;
pub mod q11_container_with_most_water;
pub mod q12_integer_to_roman;
pub mod q13_roman_to_integer;
pub mod q14_longest_common_prefix;
pub mod q15_3_sum;
pub mod q16_3_sum_closest;
pub mod q17_letter_combinations_of_a_phone_number;
pub mod q18_4_sum;
pub mod q19_remove_nth_node_from_end_of_list;
pub mod q20_valid_parentheses;
pub mod q21_merge_two_sorted_lists;
pub mod q22_generate_parentheses;
pub mod q23_merge_k_sorted_lists;
pub mod q24_swap_nodes_in_pairs;
pub mod q25_reverse_nodes_in_k_group;
pub mod q26_remove_duplicates_from_sorted_array;
pub mod q27_remove_element;
pub mod q29_divide_two_integers;
pub mod q30_substring_with_concatenation_of_all_words;
