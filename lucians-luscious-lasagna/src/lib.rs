// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const EXPECTED_OVEN_TIME: i32 = 40;
const LAYER_PREPARATION_TIME: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    return EXPECTED_OVEN_TIME;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return EXPECTED_OVEN_TIME - actual_minutes_in_oven;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    return LAYER_PREPARATION_TIME * number_of_layers;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
}
