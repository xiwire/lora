#[cfg(feature = "embedded")]
use micromath::F32Ext;

use crate::dsp::constants;
use crate::{define_params};

// 1. Define the high-level control parameters.
// mode: 0 = LowPass, 1 = HighPass, 2 = BandPass, 3 = Notch
pub enum BiquadFilterModes {
    LowPass = 0,
    HighPass = 1,
    BandPass = 2,
    Notch = 3,
}

define_params!(
    BiquadParams
    BiquadParamsState
    {
        cutoff,
        q,
        gain,
        mode,
    }
);

pub struct BiquadFilter {
    pub params: BiquadParams,
    pub state: BiquadParamsState,
    
    // Filter Memory (Delay Line)
    // x is input, y is output
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    
    sample_rate: f32,
}

impl BiquadFilter {
    pub fn new(params: BiquadParams, sample_rate: f32) -> Self {
        Self {
            params,
            state: BiquadParamsState::new(&params, 0.001),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
            sample_rate,
        }
    }

    /// Calculates the coefficients based on the current smoothed parameters.
    /// This implements the standard Robert Bristow-Johnson (RBJ) cookbook formulas.
    fn calculate_coefficients(&self) -> (f32, f32, f32, f32, f32) {
        let freq = self.state.cutoff.current;
        let q = self.state.q.current;
        let mode = self.state.mode.current.round() as i32;
        
        let omega = 2.0 * constants::PI * freq / self.sample_rate;
        let sin_w = omega.sin();
        let cos_w = omega.cos();
        let alpha = sin_w / (2.0 * q);

        let (b0, b1, b2, a0, a1, a2) = match mode {
            0 => { // Low Pass
                let b0 = (1.0 - cos_w) / 2.0;
                let b1 = 1.0 - cos_w;
                let b2 = (1.0 - cos_w) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            },
            1 => { // High Pass
                let b0 = (1.0 + cos_w) / 2.0;
                let b1 = -(1.0 + cos_w);
                let b2 = (1.0 + cos_w) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            },
            2 => { // Band Pass
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            },
            _ => { // Default to Low Pass
                let b0 = (1.0 - cos_w) / 2.0;
                let b1 = 1.0 - cos_w;
                let b2 = (1.0 - cos_w) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
        };

        // Normalize all coefficients by a0
        (
            b0 / a0,
            b1 / a0,
            b2 / a0,
            a1 / a0,
            a2 / a0,
        )
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // 1. Update smoothers for this sample
        self.state.update(&self.params);

        // 2. Recalculate coefficients based on smoothed values
        let (b0, b1, b2, a1, a2) = self.calculate_coefficients();

        // 3. Direct Form I Difference Equation
        // y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
        let output = b0 * input 
                   + b1 * self.x1 
                   + b2 * self.x2 
                   - a1 * self.y1 
                   - a2 * self.y2;

        // 4. Update delay line
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        // 5. Apply final gain
        output * self.state.gain.current
    }
}
