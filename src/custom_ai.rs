/*
    obfuscate integer -- against Cheat Engine Users.
    Copyright (C) 2021  Neutron3529

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#[macro_use]
pub mod custom_ops {
    /// Custom Initialize and Assign statement with macro `cai!`
    /// # Example
    /// ```
    /// #[macro_use] // if you want to import the macro
    /// extern crate obfuscate_integer;
    /// use obfuscate_integer::custom_ops::*;
    /// fn main(){
    ///     cai!{
    ///         let _a=1;               // a normal statement
    ///         let mut a:i32 :=0;      // Custom Initialize (let mut a:i32 =CustomInitialize::custom_initialize())
    ///         a+=1;                   // stmt
    ///         println!("{}: cai! works directly in the environment of macro",a);
    ///         for i in 0i32..1{
    ///             let j:i32 :=3;
    ///             println!("{}: cai! also works in for-loop",j-a+i)
    ///         }
    ///         a~2;                    // Custom Assign (bind to `~`)
    ///         if a==2{
    ///             let j:i32 :=1;
    ///             println!("{}: cai! also works in if statement",a+j)
    ///         }
    ///         a.custom_assign(6i32);  // expr with no ending semicolon
    ///         if a==1i32{
    ///         }else{
    ///             let j:i32 :=2;
    ///             println!("{}: cai! also works in if statement with `else` clause, empty block, etc.",a-j)
    ///         }
    ///         while a!=1{
    ///             a:=1;               // := could be used without let clause.
    ///             println!("5: cai! also works in while statement.")
    ///         }
    ///         loop{
    ///             println!("{}: cai! also works in loop statement.",a+5);
    ///             break {
    ///                 let b:i32 := 7 ;
    ///                 println!("{}: cai! also works in loop statement.",b) ;
    ///                 b
    ///             }
    ///         }
    ///         println!("{}: as a special remainder, cai does not works in closure nor blocks in a statement that is not mentioned above.",a+7)
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules! cai {
        (@expr_block ($($ex:tt)+) =>  { $($b1:tt)* } else  { $($b2:tt)* } $($tail:tt)*) =>{
            $($ex)+ {
                cai!($($b1)*);
            }else{
                cai!($($b2)*);
            }
            cai!($($tail)*)
        };
        (@expr_block ($($ex:tt)+) => { $($b1:tt)* } $($tail:tt)*) =>{
            $($ex)+ {
                cai!($($b1)*);
            }
            cai!($($tail)*)
        };
        ({ $($b1:tt)* } $($tail:tt)*) =>{
            {
                cai!($($b1)*);
            };
            cai!($($tail)*)
        };
        (@split_exp_block ($($ex:tt)+) => { $($b:tt)* } $($tail:tt)*) => {
            cai!(@expr_block ($($ex)+) => {$($b)*} $($tail)*)
        };
        (@split_exp_block ($($ex:tt)+) => $t:tt $($tail:tt)*) => {
            cai!(@split_exp_block ($($ex)+ $t) => $($tail)*)
        };
        (if $t:tt $($tail:tt)*) => {
            cai!(@split_exp_block (if $t) => $($tail)*)
        };
        (while $t:tt $($tail:tt)*) => {
            cai!(@split_exp_block (while $t) => $($tail)*)
        };
        (break { $($b:tt)* } $($tail:tt)*) => {
            break { cai!($($b)*); };
            cai!($($tail)*)
        };
        (loop { $($b:tt)* } $($tail:tt)*) => {
            loop { cai!($($b)*); } ;
            cai!($($tail)*)
        };
        (for $t:tt $($tail:tt)*) => {
            cai!(@split_exp_block (for $t) => $($tail)*)
        };
        ({ $($b:tt)* } $($tail:tt)*) => {
            { cai!($($b:tt)*) }
            cai!($($tail)*)
        };
        ($id:ident ~ $ex:expr; $($tail:tt)*) => {
            $id.custom_assign(cai!($ex));
            cai!($($tail)*)
        };
        ($($id:ident)+ $(: $type:ty)? : = $ex:expr; $($tail:tt)*) => {
            $($id)+ $(: $type)? = CustomInitialize::custom_initialize(cai!($ex));
            cai!($($tail)*)
        };
        ($st:stmt; $($tail:tt)*) => {
            $st
            cai!($($tail)*)
        };
        ($ex:expr) => {
            $ex
        };
        () => {};
    }
    /// this trait is mainly for the incoming CustomAssign stmt `a~b` implemented by my incoming marco cai!
    pub trait CustomAssign<Rhs=Self>{ // bind to a~b
        fn custom_assign(&mut self, rhs:Rhs); // a~b means a.custom_assign(b)
    }
    /// this trait is mainly for the incoming CustomInitialize stmt `let a:Type :=b` implemented by my incoming marco cai!
    pub trait CustomInitialize<Rhs=Self>{ // bind to a:=b
        fn custom_initialize(rhs:Rhs)->Self; // a:=b means a=<type of a>::custom_initialize(rhs);
    }
    impl<T> CustomAssign<T> for T{
        #[inline(always)]
        default fn custom_assign(&mut self, rhs:T){
            *self=rhs
        }
    }
    impl<T:Copy> CustomAssign<&T> for T{
        #[inline(always)]
        default fn custom_assign(&mut self, rhs:&T){
            *self=*rhs
        }
    }
    impl<'a,T:CustomAssign<&'a T>> CustomAssign<&'a mut T> for T{
        #[inline(always)]
        default fn custom_assign(&mut self, rhs:&'a mut T){
            self.custom_assign(rhs as &T)
        }
    }
    impl<T> CustomInitialize<T> for T{
        #[inline(always)]
        default fn custom_initialize(rhs:T)->Self{
            rhs
        }
    }
    impl<T:Copy> CustomInitialize<&T> for T{
        #[inline(always)]
        default fn custom_initialize(rhs:&T)->Self{
            *rhs
        }
    }
    impl<'a,T:CustomInitialize<&'a T>> CustomInitialize<&'a mut T> for T{
        #[inline(always)]
        default fn custom_initialize(rhs:&'a mut T)->Self{
            Self::custom_initialize(rhs as &T)
        }
    }
}
pub mod prelude {
    pub use super::{Oi32,Ei32,Ou32,Eu32,custom_ops::*};
}
