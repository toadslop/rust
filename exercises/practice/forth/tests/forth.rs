#[test]
fn numbers_just_get_pushed_onto_the_stack() {
    let input = {"instructions":["1 2 3 4 5"]};
    let output = forth::new(input);
    let expected = [1,2,3,4,5];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn pushes_negative_numbers_onto_the_stack() {
    let input = {"instructions":["-1 -2 -3 -4 -5"]};
    let output = forth::new(input);
    let expected = [-1,-2,-3,-4,-5];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_add_two_numbers() {
    let input = {"instructions":["1 2 +"]};
    let output = forth::new(input);
    let expected = [3];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["+"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 +"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_subtract_two_numbers() {
    let input = {"instructions":["3 4 -"]};
    let output = forth::new(input);
    let expected = [-1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["-"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 -"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_multiply_two_numbers() {
    let input = {"instructions":["2 4 *"]};
    let output = forth::new(input);
    let expected = [8];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["*"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 *"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_divide_two_numbers() {
    let input = {"instructions":["12 3 /"]};
    let output = forth::new(input);
    let expected = [4];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn performs_integer_division() {
    let input = {"instructions":["8 3 /"]};
    let output = forth::new(input);
    let expected = [2];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_dividing_by_zero() {
    let input = {"instructions":["4 0 /"]};
    let output = forth::new(input);
    let expected = {"error":"divide by zero"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["/"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 /"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn addition_and_subtraction() {
    let input = {"instructions":["1 2 + 4 -"]};
    let output = forth::new(input);
    let expected = [-1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn multiplication_and_division() {
    let input = {"instructions":["2 4 * 3 /"]};
    let output = forth::new(input);
    let expected = [2];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn copies_a_value_on_the_stack() {
    let input = {"instructions":["1 dup"]};
    let output = forth::new(input);
    let expected = [1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn copies_the_top_value_on_the_stack() {
    let input = {"instructions":["1 2 dup"]};
    let output = forth::new(input);
    let expected = [1,2,2];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["dup"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn removes_the_top_value_on_the_stack_if_it_is_the_only_one() {
    let input = {"instructions":["1 drop"]};
    let output = forth::new(input);
    let expected = [];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
    let input = {"instructions":["1 2 drop"]};
    let output = forth::new(input);
    let expected = [1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["drop"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn swaps_the_top_two_values_on_the_stack_if_they_are_the_only_ones() {
    let input = {"instructions":["1 2 swap"]};
    let output = forth::new(input);
    let expected = [2,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
    let input = {"instructions":["1 2 3 swap"]};
    let output = forth::new(input);
    let expected = [1,3,2];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["swap"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 swap"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn copies_the_second_element_if_there_are_only_two() {
    let input = {"instructions":["1 2 over"]};
    let output = forth::new(input);
    let expected = [1,2,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn copies_the_second_element_if_there_are_more_than_two() {
    let input = {"instructions":["1 2 3 over"]};
    let output = forth::new(input);
    let expected = [1,2,3,2];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_nothing_on_the_stack() {
    let input = {"instructions":["over"]};
    let output = forth::new(input);
    let expected = {"error":"empty stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_there_is_only_one_value_on_the_stack() {
    let input = {"instructions":["1 over"]};
    let output = forth::new(input);
    let expected = {"error":"only one value on the stack"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_consist_of_built_in_words() {
    let input = {"instructions":[": dup-twice dup dup ;","1 dup-twice"]};
    let output = forth::new(input);
    let expected = [1,1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn execute_in_the_right_order() {
    let input = {"instructions":[": countup 1 2 3 ;","countup"]};
    let output = forth::new(input);
    let expected = [1,2,3];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_override_other_user_defined_words() {
    let input = {"instructions":[": foo dup ;",": foo dup dup ;","1 foo"]};
    let output = forth::new(input);
    let expected = [1,1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_override_built_in_words() {
    let input = {"instructions":[": swap dup ;","1 swap"]};
    let output = forth::new(input);
    let expected = [1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_override_built_in_operators() {
    let input = {"instructions":[": + * ;","3 4 +"]};
    let output = forth::new(input);
    let expected = [12];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_use_different_words_with_the_same_name() {
    let input = {"instructions":[": foo 5 ;",": bar foo ;",": foo 6 ;","bar foo"]};
    let output = forth::new(input);
    let expected = [5,6];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_define_word_that_uses_word_with_the_same_name() {
    let input = {"instructions":[": foo 10 ;",": foo foo 1 + ;","foo"]};
    let output = forth::new(input);
    let expected = [11];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn cannot_redefine_non_negative_numbers() {
    let input = {"instructions":[": 1 2 ;"]};
    let output = forth::new(input);
    let expected = {"error":"illegal operation"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn cannot_redefine_negative_numbers() {
    let input = {"instructions":[": -1 2 ;"]};
    let output = forth::new(input);
    let expected = {"error":"illegal operation"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn errors_if_executing_a_non_existent_word() {
    let input = {"instructions":["foo"]};
    let output = forth::new(input);
    let expected = {"error":"undefined operation"};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn only_defines_locally() {
    let input = {"instructionsFirst":[": + - ;","1 1 +"],"instructionsSecond":["1 1 +"]};
    let output = forth::new(input);
    let expected = [[0],[2]];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn dup_is_case_insensitive() {
    let input = {"instructions":["1 DUP Dup dup"]};
    let output = forth::new(input);
    let expected = [1,1,1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn drop_is_case_insensitive() {
    let input = {"instructions":["1 2 3 4 DROP Drop drop"]};
    let output = forth::new(input);
    let expected = [1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn swap_is_case_insensitive() {
    let input = {"instructions":["1 2 SWAP 3 Swap 4 swap"]};
    let output = forth::new(input);
    let expected = [2,3,4,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn over_is_case_insensitive() {
    let input = {"instructions":["1 2 OVER Over over"]};
    let output = forth::new(input);
    let expected = [1,2,1,2,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn user_defined_words_are_case_insensitive() {
    let input = {"instructions":[": foo dup ;","1 FOO Foo foo"]};
    let output = forth::new(input);
    let expected = [1,1,1,1];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn definitions_are_case_insensitive() {
    let input = {"instructions":[": SWAP DUP Dup dup ;","1 swap"]};
    let output = forth::new(input);
    let expected = [1,1,1,1];
    assert_eq!(output, expected);
}
