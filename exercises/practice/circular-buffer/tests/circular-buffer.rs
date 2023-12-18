use circular_buffer::*;


#[test]
fn reading_empty_buffer_should_fail() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"read","should_succeed":false}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_read_an_item_just_written() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn each_item_may_only_be_read_once() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1},{"operation":"read","should_succeed":false}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn items_are_read_in_the_order_they_are_written() {
    let mut buffer = CircularBuffer::new(2);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1},{"operation":"read","should_succeed":true,"expected":2}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn full_buffer_can_t_be_written_to() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":false}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_read_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1},{"operation":"write","item":2,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":2}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn read_position_is_maintained_even_across_multiple_writes() {
    let mut buffer = CircularBuffer::new(3);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1},{"operation":"write","item":3,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":2},{"operation":"read","should_succeed":true,"expected":3}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn items_cleared_out_of_buffer_can_t_be_read() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"clear"},{"operation":"read","should_succeed":false}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn clear_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"clear"},{"operation":"write","item":2,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":2}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn clear_does_nothing_on_empty_buffer() {
    let mut buffer = CircularBuffer::new(1);
    let operations = [{"operation":"clear"},{"operation":"write","item":1,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn overwrite_acts_like_write_on_non_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"overwrite","item":2},{"operation":"read","should_succeed":true,"expected":1},{"operation":"read","should_succeed":true,"expected":2}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn overwrite_replaces_the_oldest_item_on_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":true},{"operation":"overwrite","item":3},{"operation":"read","should_succeed":true,"expected":2},{"operation":"read","should_succeed":true,"expected":3}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn overwrite_replaces_the_oldest_item_remaining_in_buffer_following_a_read() {
    let mut buffer = CircularBuffer::new(3);
    let operations = [{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":true},{"operation":"write","item":3,"should_succeed":true},{"operation":"read","should_succeed":true,"expected":1},{"operation":"write","item":4,"should_succeed":true},{"operation":"overwrite","item":5},{"operation":"read","should_succeed":true,"expected":3},{"operation":"read","should_succeed":true,"expected":4},{"operation":"read","should_succeed":true,"expected":5}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn initial_clear_does_not_affect_wrapping_around() {
    let mut buffer = CircularBuffer::new(2);
    let operations = [{"operation":"clear"},{"operation":"write","item":1,"should_succeed":true},{"operation":"write","item":2,"should_succeed":true},{"operation":"overwrite","item":3},{"operation":"overwrite","item":4},{"operation":"read","should_succeed":true,"expected":3},{"operation":"read","should_succeed":true,"expected":4},{"operation":"read","should_succeed":false}];
    let output = new(input);
    let expected = {};
    assert_eq!(output, expected);
}
