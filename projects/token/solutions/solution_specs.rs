#![no_std]
use soroban_sdk::{Address, Env};

use crate::Token;

use cvlr_soroban_derive::rule;
use cvlr::asserts::{cvlr_assert, cvlr_assume, cvlr_satisfy};

use certora_soroban::{certora_print_i64, CERTORA_calltrace_print_c_i64, is_auth};


// Sunbeam specs

// Exercise 0
#[rule]
fn sanity(e: Env, addr: Address) {
    let balance = Token::balance(&e, addr);
    // Reachability check: is this satisfy reachable?
    cvlr_satisfy!(true);
}

// Exercise 1
#[rule]
fn init_balance(e: Env, addr: Address) {
    // precondition macro
    cvlr_assume!(!e.storage().persistent().has(&addr));
    let balance = Token::balance(&e, addr);
    // use this macro to see additional information in the calltrace
    certora_print_i64!("value of balance is:", balance); 
    // postcondition macro
    cvlr_assert!(balance == 0);
}


// Exercise 2
#[rule]
fn transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    cvlr_assume!(
        e.storage().persistent().has(&from) && e.storage().persistent().has(&to) && to != from);
    let balance_from_before = Token::balance(&e, from.clone());
    let balance_to_before = Token::balance(&e, to.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_from_after = Token::balance(&e, from.clone());
    let balance_to_after = Token::balance(&e, to.clone());
    cvlr_assert!(
        (balance_to_after == balance_to_before + amount)
            && (balance_from_after == balance_from_before - amount)
    );
}

// Exercise 2
#[rule]
fn transfer_no_effect_on_other(e: Env, amount: i64, from: Address, to: Address, other: Address) {
    cvlr_assume!(to != other && from != other);
    let balance_other_before = Token::balance(&e, other.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_other_after = Token::balance(&e, other.clone());
    cvlr_assert!(balance_other_after == balance_other_before);
}

// Exercise 3
#[rule]
fn transfer_fails_if_low_balance(e: Env, to: Address, from: Address, amount: i64) {
    cvlr_assume!(
        e.storage().persistent().has(&from)
            && e.storage().persistent().has(&to)
            && to != from
            && Token::balance(&e, from.clone()) < amount);
    Token::transfer(&e, from.clone(), to.clone(), amount);
    // should not reach and therefore rule must pass
    cvlr_assert!(false);
}

// Exercise 4
#[rule]
fn burn_is_correct(e: Env, from: Address, amount: i64) {
    let balance_before = Token::balance(&e, from.clone());
    Token::burn(&e, from.clone(), amount);
    let balance_after = Token::balance(&e, from.clone());
    cvlr_assert!(balance_after == balance_before - amount);
}

// Exercise 4
#[rule]
fn mint_is_authorized(e: Env, to: Address, amount: i64) {
    let admin = e
            .storage()
            .persistent()
            .get::<_, Address>(&"ADMIN")
            .unwrap();
    cvlr_assume!(is_auth(admin));
    Token::mint(&e, to, amount);
    cvlr_satisfy!(true);
}

// Exercise 4
#[rule]
fn mint_not_authorized_fails(e: Env, to: Address, amount: i64) {
    let admin = e
            .storage()
            .persistent()
            .get::<_, Address>(&"ADMIN")
            .unwrap();
    cvlr_assume!(!is_auth(admin));
    Token::mint(&e, to, amount);
    cvlr_assert!(false); // should pass
}

// Exercise 4
#[rule]
fn mint_is_correct(e: Env, to: Address, amount: i64) {
    let balance_before = Token::balance(&e, to.clone());
    Token::mint(&e, to.clone(), amount);
    let balance_after = Token::balance(&e, to.clone());
    cvlr_assert!(balance_after == balance_before + amount);
}