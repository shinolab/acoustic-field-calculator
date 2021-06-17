/*
 * File: greedy.rs
 * Project: combination
 * Created Date: 17/06/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/06/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use crate::*;

pub struct Greedy {
    foci: Vec<Vector3>,
    amps: Vec<Float>,
    phases: Vec<Complex>,
}

impl Greedy {
    pub fn new(foci: Vec<Vector3>, amps: Vec<Float>, phase_div: usize) -> Self {
        let mut phases = Vec::with_capacity(phase_div);
        for i in 0..phase_div {
            phases.push(Complex::new(0., 2.0 * PI * i as Float / phase_div as Float).exp());
        }
        Self { foci, amps, phases }
    }
}

impl Optimizer for Greedy {
    #[allow(clippy::many_single_char_names)]
    #[allow(non_snake_case)]
    fn optimize<S: WaveSource>(&self, system: &mut UniformSystem<S>) {
        for source in system.wave_sources_mut() {
            source.set_phase(0.);
        }

        let m = self.foci.len();

        let mut tmp = Vec::with_capacity(self.phases.len());
        tmp.resize(self.phases.len(), vec![Complex::new(0., 0.); m]);

        let mut cache = Vec::with_capacity(m);
        cache.resize(m, Complex::new(0., 0.));

        fn transfer_foci<S: WaveSource>(
            system: &mut UniformSystem<S>,
            source_idx: usize,
            phase: Complex,
            foci: &[Vector3],
            res: &mut [Complex],
        ) {
            for i in 0..foci.len() {
                res[i] = system.propagate(source_idx, foci[i]) * phase;
            }
        }

        for i in 0..system.wave_sources().len() {
            let mut min_idx = 0;
            let mut min_v = Float::INFINITY;
            for (idx, &phase) in self.phases.iter().enumerate() {
                transfer_foci(system, i, phase, &self.foci, &mut tmp[idx]);
                let mut v = 0.0;
                for (j, c) in cache.iter().enumerate() {
                    v += (self.amps[j] - (tmp[idx][j] + c).abs()).abs();
                }

                if v < min_v {
                    min_v = v;
                    min_idx = idx;
                }
            }

            for (j, c) in cache.iter_mut().enumerate() {
                *c += tmp[min_idx][j];
            }

            let phase = self.phases[min_idx].argument();
            system.wave_sources_mut()[i].set_phase(phase);
        }
    }
}
