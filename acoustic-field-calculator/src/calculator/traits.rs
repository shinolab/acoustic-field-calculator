/*
* File: trait.rs
* Project: calculator
* Created Date: 19/11/2020
* Author: Shun Suzuki
* -----
* Last Modified: 19/11/2020
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2020 Hapis Lab. All rights reserved.
*
*/

/// Calculate field at observe area
///
/// # Arguments
///
/// * `medium` - Propagation medium
/// * `observe_area` - Observation area
/// * `buffer` - Buffer which contains results and define result field type
///
pub trait FieldCalculator<S, M, A, O, F> {
    fn calculate(&self, medium: &M, observe_area: &A, buffer: &mut F);
}
