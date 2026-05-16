/// Generic parameters

/// Generic access for MIDI/GUI
pub trait ParamIndexed {
    fn get_value(&self, index: usize) -> f32;
    fn set_value(&mut self, index: usize, value: f32);
    fn len(&self) -> usize;
}

/// A value that slides smoothly toward a target
#[derive(Debug, Clone, Copy)]
pub struct SmoothedValue {
    pub current: f32,
    pub coefficient: f32,
}

impl SmoothedValue {
    pub const fn new(initial: f32, coefficient: f32) -> Self {
        Self { current: initial, coefficient }
    }

    #[inline(always)]
    pub fn next(&mut self, target: f32) -> f32 {
        self.current += self.coefficient * (target - self.current);
        self.current
    }
}

pub trait PodParams 
where
    Self: Sized
{
    #[inline(always)]
    fn as_slice(&self) -> &[f32] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const f32, core::mem::size_of::<Self>() / 4) }
    }
    #[inline(always)]
    fn as_slice_mut(&mut self) -> &mut [f32] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut f32, core::mem::size_of::<Self>() / 4) }
    }
}

#[macro_export]
macro_rules! define_params {
    ($name:ident $state_name:ident { $($field:ident),* $(,)? }) => {
        // 1. Target Struct: All fields are automatically f32.
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            $( pub $field: f32, )*
        }

        // Automatically implement PodParams for the generated struct
        impl $crate::dsp::param::PodParams for $name {}

        // 2. State Struct: Smoothed values matching the targets.
        #[derive(Debug, Clone, Copy)]
        pub struct $state_name {
            $( pub $field: $crate::dsp::param::SmoothedValue, )*
        }

        impl $state_name {
            pub fn new(params: &$name, coeff: f32) -> Self {
                Self {
                    $( $field: $crate::dsp::param::SmoothedValue::new(params.$field, coeff), )*
                }
            }

            #[inline(always)]
            pub fn update(&mut self, params: &$name) {
                $(
                    self.$field.next(params.$field);
                )*
            }
        }
    };
}
