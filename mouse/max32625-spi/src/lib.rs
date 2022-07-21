#![no_std]
pub use spim::Spim;
pub use trans::{Polarity, SpiClockScale, SpiError, SpiMode, SpiPageSize, SpiSizeUnits, SS};

macro_rules! getter_setter_impl {
    ($name:ident<$($gen:ident: $typ:tt),*> :
        $($vis:vis $field_name:ident, $setter_name:ident: $field_type:ty,)*
    ) => {
        impl<$($gen: $typ),*> $name<$($gen),*> {
            $(
                $vis fn $field_name(&self) -> &$field_type {
                    &self.$field_name
                }

                $vis fn $setter_name(&mut self, $field_name: $field_type) {
                    self.$field_name = $field_name;
                }
            )*
        }
    };
}

macro_rules! impl_spims {
    ($name:ident<$($lifes:lifetime),*$($gens:ident: $gen_ts:tt),*> $gen_name:ident : mod $mod_name:ident $($code:tt)*) => {
        mod $mod_name {
            use super::*;

            mod __spim0 {
                use super::*;
                type $gen_name = max32625::SPIM0;

                impl<$($lifes,)*$($gens: $gen_ts,)*> $name<$($lifes,)*$($gens,)*max32625::SPIM0> {
                    $($code)*
                }
            }
            mod __spim1 {
                use super::*;
                type $gen_name = max32625::SPIM1;

                impl<$($lifes,)*$($gens: $gen_ts,)*> $name<$($lifes,)*$($gens,)*max32625::SPIM1> {
                    $($code)*
                }
            }
            mod __spim2 {
                use super::*;
                type $gen_name = max32625::SPIM2;

                impl<$($lifes,)*$($gens: $gen_ts,)*> $name<$($lifes,)*$($gens,)*max32625::SPIM2> {
                    $($code)*
                }
            }
        }
    };
}

macro_rules! unique_impl_spims {
    (
        $struct_name:ident
        $(
            $fun_vis:vis fn $fun_name:ident($(& $self:tt)? $(,)? $($arg_name:ident: $arg_type:ty),*) $(-> $fun_ret_type:ty)? {
            where ($($var_names:tt)+) =
                SPI0 => ($($spi0_values:tt)+)
                SPI1 => ($($spi1_values:tt)+)
                SPI2 => ($($spi2_values:tt)+)

                $($code:tt)*
            }
        )+
    ) => {
        impl $struct_name<max32625::SPIM0> {
            $(
                $fun_vis fn $fun_name($(&$self,)?$($arg_name: $arg_type),*) $(-> $fun_ret_type)? {
                    #[allow(unused_parens)]
                    let ($($var_names)+) = ($($spi0_values)+);

                    $($code)*
                }
            )+
        }
        impl $struct_name<max32625::SPIM1> {
            $(
                $fun_vis fn $fun_name($(&$self,)?$($arg_name: $arg_type),*) $(-> $fun_ret_type)? {
                    #[allow(unused_parens)]
                    let ($($var_names)+) = ($($spi1_values)+);

                    $($code)*
                }
            )+
        }
        impl $struct_name<max32625::SPIM2> {
            $(
                $fun_vis fn $fun_name($(&$self,)?$($arg_name: $arg_type),*) $(-> $fun_ret_type)? {
                    #[allow(unused_parens)]
                    let ($($var_names)+) = ($($spi2_values)+);

                    $($code)*
                }
            )+
        }
    };
}

mod spim;
mod trans;
