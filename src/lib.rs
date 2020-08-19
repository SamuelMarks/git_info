#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bindings_with_variant_name,
    broken_intra_doc_links,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_docs,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # git_info
//!
//! Extracts git repository information.
//!
//! This library main goal is to provide development/build tools such as
//! [cargo-make](https://sagiegurari.github.io/cargo-make/)the needed information on the current git repository.
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let info = git_info::get();
//!
//!     println!("User Name: {}", info.user_name.unwrap_or("Unknown".to_string()));
//!     println!("User Email: {}", info.user_email.unwrap_or("Unknown".to_string()));
//!     println!("Dirty: {}", info.dirty.unwrap_or(false));
//!     println!("Current Branch: {}", info.current_branch.unwrap_or("Unknown".to_string()));
//!     println!("Last Commit Hash: {}", info.head.last_commit_hash.unwrap_or("Unknown".to_string()));
//!     println!("Last Commit Hash (short): {}", info.head.last_commit_hash_short.unwrap_or("Unknown".to_string()));
//!     println!("Config: {:#?}", info.config.unwrap());
//!     println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! git_info = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/git_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/git_info/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod gitinfo;
pub mod types;

use crate::types::GitInfo;

/// Returns the current git repository information.
///
/// # Example
///
/// ```
/// fn main() {
///     let info = git_info::get();
///
///     println!("User Name: {}", info.user_name.unwrap_or("Unknown".to_string()));
///     println!("User Email: {}", info.user_email.unwrap_or("Unknown".to_string()));
///     println!("Dirty: {}", info.dirty.unwrap_or(false));
///     println!("Current Branch: {}", info.current_branch.unwrap_or("Unknown".to_string()));
///     println!("Last Commit Hash: {}", info.head.last_commit_hash.unwrap_or("Unknown".to_string()));
///     println!("Last Commit Hash (short): {}", info.head.last_commit_hash_short.unwrap_or("Unknown".to_string()));
///     println!("Config: {:#?}", info.config.unwrap());
///     println!("Branches: {:#?}", info.branches.unwrap_or(vec![]));
/// }
/// ```
pub fn get() -> GitInfo {
    gitinfo::get()
}
