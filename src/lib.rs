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
//! This is a crate for test purpose, which may stop most Cheat Engine (and other variants)
//! program scanning for the relative address, and thus stop further modification.
//! most of the type here provided start with an 'O' and the remain parts are the same to its actual kind
//! e.g., `Oi32` is actually an obfuscate `i32`
//! actually for most Cheat Engine user, it is difficult even modify a very simple program with Oi32
//! # license: GPL-v3 or later
//! # Examples
//! ```no_run
//! #![feature(bench_black_box)]
//! use std::hint::black_box;
//! #[macro_use] // if you want to import the macro
//! extern crate obfuscate_integer;
//! use obfuscate_integer::*;
//! fn main(){cai!{
//!     let mut player_hp:Oi32:=500000;
//!     let mut enemy_hp:Oi32:=1000000;
//!     let mut round:Oi16:=0;
//!     let now=std::time::Instant::now();
//!     loop{
//!         round+=1;
//!         enemy_hp-=1;
//!         if enemy_hp<0 {break}
//!         player_hp-=1;
//!         if player_hp<0 {break}
//!     }
//!     println!("execute 5000000 loops, cost {:?}",now.elapsed());
//!     let mut player_hp:Oi32:=500000;
//!     let mut enemy_hp:Oi32:=1000000;
//!     let mut round:Oi16:=0;
//!     let now=std::time::Instant::now();
//!     loop{
//!         round+=black_box(1);
//!         enemy_hp-=black_box(1);
//!         if enemy_hp<0 {break}
//!         player_hp-=black_box(1);
//!         if player_hp<0 {break}
//!     }
//!     println!("execute 5000000 loops, cost {:?}",now.elapsed());
//!     let mut stdin = std::io::stdin();
//!     let mut player_hp:Oi32:=500000;
//!     let mut enemy_hp:Oi32:=1000000;
//!     let mut round:Oi16:=0;
//!     let mut buffer = String::new();
//!     } // at least 0.1.0, `cai!` does not support complicated instructions like the following one. it might be solved in future versions.
//!     if loop{
//!         round+=1;
//!         println!("Round {} is comming, player's hp is {} and enemy's hp is {}. Press `Enter` to continue.",&round,&player_hp,&enemy_hp);
//!         stdin.read_line(&mut buffer).unwrap();
//!         buffer.clear();
//!         enemy_hp-=1;
//!         if enemy_hp<0 {break true}
//!         player_hp-=1;
//!         if player_hp<0 {break false}
//!     }{
//!         println!("You win!");
//!     }else{
//!         println!("You dead.");
//!     }
//! }
//! ```
//! # Restrictions
//! program must be compiled with `overflow-checks = false`, since Debug mode could not handle the highly possible wrapping ops.
//! using std::hint::black_box, it is not difficult figure out that Oi32 makes program run 100x slower than using i32
//! thus, do not use Oi* or Ou* dealing massive calculations.
//! for normal (e.g., gaming) use, that is acceptable.
#![feature(specialization)]
#![feature(box_syntax)]
#![allow(overflowing_literals)]
include!("custom_ai.rs");
pub use crate::custom_ops::{CustomInitialize,CustomAssign};

macro_rules! Oint_impl {
    ($SelfT:ident,$SelfE:ident,$ActualT:ty,$OET:ident)=>{
        pub mod $OET{
            use std::ops::*;
            use crate::custom_ops::{CustomInitialize,CustomAssign};
            #[cfg(not(feature = "forbid-unsafe"))]
            static mut MAGIC:$ActualT=(7895123i128 | (7895123i128<<60)) as $ActualT;
            #[doc = concat!(stringify!($SelfE)," is the basic storage unit of ",stringify!($SelfT))]
            /// it would prevent Cheat Engine from figure out what is changed, thus stop Cheat Engine
            /// modify the value.
            /// # Warning:
            /// This might be a quite early version, all the `pub` flag may be changed in the future
            /// since its performance is 100x slower than using the primitive type
            /// for 0.1.0 users, please do not update to 0.2.x since that could be a breaking change.
            /// Do not feel surprise if you find the version number become 0.1.3511
            #[derive(Clone,Debug)]
            pub enum $SelfE {
                SS($ActualT,$ActualT),
                SC($ActualT,$SelfT),
                CS($SelfT,$ActualT),
                CC($SelfT,$SelfT)
            }
            #[doc = concat!(stringify!($SelfT)," is the obfuscate version of ",stringify!($ActualT))]
            /// it acts quite like the original primitive type, but will prevent most Cheat Engine user
            /// from modifying it unless they are very familar with both Rust program and your code.
            /// # Warning:
            /// This might be a quite early version, all the `pub` flag may be changed in the future
            /// since its performance is 100x slower than using the primitive type
            /// for 0.1.0 users, please do not update to 0.2.x since that could be a breaking change.
            /// Do not feel surprise if you find the version number become 0.1.3511
            #[derive(Clone,Debug)]
            pub struct $SelfT{
                pub status:$ActualT,
                pub val:Box<$SelfE>
            }
            impl $SelfE{
                #[inline(always)]
                #[cfg(feature = "forbid-unsafe")]
                fn magic()->$ActualT{let a=();&a as *const () as $ActualT}
                #[cfg(not(feature = "forbid-unsafe"))]
                fn _magic()->$ActualT{let a=();&a as *const () as $ActualT}
                #[cfg(not(feature = "forbid-unsafe"))]
                fn magic()->$ActualT{
                    let c=unsafe {
                        // SAFETY: MAGIC does not contains any useful data, thus could be a good RNG
                        &mut MAGIC
                    };
                    let d=*c;
                    *c=d*(d+1)*6+1;
                    if d==*c{
                        *c=Self::_magic()
                    }
                    d
                }
                #[inline(always)]
                fn new_basic(val:$ActualT)->Self{let m=Self::magic().wrapping_mul(val);Self::SS(m,val^m)}
                #[inline(always)]
                fn new_sc(val:$ActualT,c2:$SelfT)->Self{Self::SC(val^c2.value(),c2)}
                #[inline(always)]
                fn new_cs(mut val:$ActualT,c1:$SelfT)->Self{val^=c1.value();Self::CS(c1,val)}
                #[inline(always)]
                fn new_cc(c1:$SelfT,c2:$SelfT)->Self{Self::CC(c1,c2)}
                #[inline(always)]
                fn value(&self)->$ActualT{
                    match self{
                        Self::SS(a,b)=>a^b,
                        Self::SC(a,b)=>a^b.value(),
                        Self::CS(a,b)=>a.value()^b,
                        Self::CC(a,b)=>a.value()^b.value()
                    }
                }
            }
            impl $SelfT{
                pub const MAGIC:$ActualT=(7895123i128 | (7895123i128<<60)) as $ActualT;
                #[inline(always)]
                pub fn new_basic(val:$ActualT)->Self{
                    let m=$SelfE::magic()^val;
                    Self{status:m^val,val:Box::new($SelfE::new_basic(val))}
                }
                #[inline(always)]
                pub fn new_cs(val:$ActualT,c1:Self)->Self{
                    let m=$SelfE::magic()^Self::MAGIC;
                    Self{status:m^val,val:Box::new($SelfE::new_cs(val,c1))}
                }
                #[inline(always)]
                pub fn new_sc(val:$ActualT,c2:Self)->Self{
                    let m=$SelfE::magic()^Self::MAGIC;
                    Self{status:m^val,val:Box::new($SelfE::new_sc(val,c2))}
                }
                #[inline(always)]
                pub fn new(val:$ActualT)->Self{
                    let m=$SelfE::magic()^Self::MAGIC;
                    Self{status:m^val,val:Box::new($SelfE::new_cc(Self::new_basic(val^m),Self::new_basic(m)))}
                }
                #[inline(always)]
                pub fn new_cssc(val:$ActualT)->Self{
                    let m=$SelfE::magic()^Self::MAGIC;
                    Self{status:m^val,val:Box::new($SelfE::new_cc(Self::new_cs(val^m,Self::new_basic(m^val^Self::MAGIC)),Self::new_sc(m,Self::new_basic(val^Self::MAGIC))))}
                }
                #[inline(always)]
                pub fn from_raw(tuple:($ActualT,$SelfE))->Self{
                    Self{status:tuple.0,val:box tuple.1}
                }
                #[inline(always)]
                pub fn into_raw(self)->($ActualT,$SelfE){
                    (self.status,*self.val)
                }
                #[inline(always)]
                pub fn value(&self)->$ActualT{
                    self.val.value()
                }
                pub fn assign(&mut self, mut val:$ActualT){
                    if self.status&31==0{
                        let tmp=3511 as $ActualT+self.value().wrapping_mul(Self::MAGIC);
                        self.status+=tmp&(-32i128) as $ActualT;
                        val^=self.value()^tmp;
                        match *self.val{
                            $SelfE::SS(ref mut a,ref mut b)=>{*a^=val;*b^=tmp},
                            $SelfE::SC(ref mut a,ref mut b)|$SelfE::CS(ref mut b,ref mut a)=>{*a^=val;b.assign(tmp^b.value())}
                            $SelfE::CC(ref mut a,ref mut b)=>{a.assign(val^a.value());b.assign(tmp^b.value())}
                        }
                    }else{
                        val^=self.value();
                        if self.status&32==0 {
                            match *self.val{
                                $SelfE::SS(ref mut a,_)|$SelfE::SC(ref mut a,_)=>*a^=val,
                                $SelfE::CS(ref mut a,_)|$SelfE::CC(ref mut a,_)=>a.assign(val^a.value())
                            }
                        }else{
                            match *self.val{
                                $SelfE::SS(_,ref mut a)|$SelfE::CS(_,ref mut a)=>*a^=val,
                                $SelfE::SC(_,ref mut a)|$SelfE::CC(_,ref mut a)=>a.assign(val^a.value())
                            }
                        }
                    }
                    self.status+=Self::MAGIC
                }
            }
            impl std::fmt::Display for $SelfT{
                #[inline(always)]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
                    self.value().fmt(f)
                }
            }
            impl CustomAssign<$ActualT> for $SelfT{
                #[inline(always)]
                fn custom_assign(&mut self,rhs:$ActualT){
                    self.assign(rhs);
                }
            }
            impl CustomInitialize<$ActualT> for $SelfT{
                #[inline(always)]
                fn custom_initialize(rhs:$ActualT)->Self{
                    Self::new(rhs)
                }
            }
            default impl<T:Into<$ActualT>> CustomAssign<T> for $SelfT{
                #[inline(always)]
                default fn custom_assign(&mut self,rhs:T){
                    self.assign(rhs.into());
                }
            }
            default impl<T:Into<$ActualT>> CustomInitialize<T> for $SelfT{
                #[inline(always)]
                default fn custom_initialize(rhs:T)->Self{
                    Self::new(rhs.into())
                }
            }
            macro_rules! assign_impl {
                ($AssignTrait:ident,$assign_op:ident,$OpTrait:ident,$op:ident)=>{
                    impl<T> $AssignTrait<T> for $SelfT where $ActualT: $OpTrait<T,Output=$ActualT>{
                        #[inline(always)]
                        fn $assign_op(&mut self,rhs:T){
                            self.assign(self.value().$op(rhs))
                        }
                    }/*
                    impl<T> $AssignTrait<T> for $SelfT where std::num::Wrapping<$ActualT>: $OpTrait<std::num::Wrapping<T>,Output=std::num::Wrapping<$ActualT>>{
                        fn $assign_op(&mut self,rhs:T){
                            self.assign(std::num::Wrapping(self.value()).$op(std::num::Wrapping(rhs)).0)
                        }
                    }*/
                    impl $AssignTrait for $SelfT{
                        #[inline(always)]
                        fn $assign_op(&mut self,rhs:Self){
                            self.assign(self.value().$op(rhs.value()))
                        }
                    }
                    impl $AssignTrait<&$SelfT> for $SelfT{
                        #[inline(always)]
                        fn $assign_op(&mut self,rhs:&Self){
                            self.assign(self.value().$op(rhs.value()))
                        }
                    }
                    macro_rules! op_impl{
                        ($lhs:ty,$rhs:ty)=>{
                            impl $OpTrait<$rhs> for $lhs{
                                type Output=$ActualT;
                                #[inline(always)]
                                fn $op(self,rhs:$rhs)->Self::Output{self.value().$op(rhs.value())}
                            }
                        }
                    }
                    macro_rules! actual_op_deref_impl{
                        ($lhs:ty,$rhs:ty)=>{
                            impl $OpTrait<$rhs> for $lhs{
                                type Output=$ActualT;
                                #[inline(always)]
                                fn $op(self,rhs:$rhs)->Self::Output{self.value().$op(*rhs)}
                            }
                        }
                    }
                    macro_rules! actual_op_impl{
                        ($lhs:ty,$rhs:ty)=>{
                            impl $OpTrait<$rhs> for $lhs{
                                type Output=$ActualT;
                                #[inline(always)]
                                fn $op(self,rhs:$rhs)->Self::Output{self.value().$op(rhs)}
                            }
                        }
                    }
                    macro_rules! lhs_ops_impl{
                        ($lhs:ty,$rhs:ty,$marco:tt)=>{
                            $marco!{$lhs,$rhs}
                            $marco!{&$lhs,$rhs}
                            $marco!{&mut $lhs,$rhs}
                        }
                    }
                    macro_rules! both_ops_impl{
                        ($lhs:ty,$rhs:ty,$marco:tt)=>{
                            lhs_ops_impl!{$lhs,$rhs,$marco}
                            lhs_ops_impl!{$lhs,&$rhs,$marco}
                            lhs_ops_impl!{$lhs,&mut $rhs,$marco}
                        }
                    }
                    both_ops_impl!($SelfT,$SelfT,op_impl);
                    lhs_ops_impl!{$SelfT,$ActualT,actual_op_impl}
                    lhs_ops_impl!{$SelfT,&$ActualT,actual_op_deref_impl}
                    lhs_ops_impl!{$SelfT,&mut $ActualT,actual_op_deref_impl}
                }
            }
            assign_impl!{AddAssign,add_assign,Add,add}
            assign_impl!{SubAssign,sub_assign,Sub,sub}
            assign_impl!{MulAssign,mul_assign,Mul,mul}
            assign_impl!{DivAssign,div_assign,Div,div}
            assign_impl!{RemAssign,rem_assign,Rem,rem}
            assign_impl!{ShlAssign,shl_assign,Shl,shl}
            assign_impl!{ShrAssign,shr_assign,Shr,shr}
            assign_impl!{BitAndAssign,bitand_assign,BitAnd,bitand}
            assign_impl!{BitOrAssign,bitor_assign,BitOr,bitor}
            assign_impl!{BitXorAssign,bitxor_assign,BitXor,bitxor}
            impl std::cmp::PartialEq for $SelfT{
                fn eq(&self,rhs:&Self)->bool{self.value()==rhs.value()}
                fn ne(&self,rhs:&Self)->bool{self.value()!=rhs.value()}
            }
            impl std::cmp::Eq for $SelfT{}
            impl std::cmp::PartialEq<$ActualT> for $SelfT{
                fn eq(&self,rhs:&$ActualT)->bool{self.value()==*rhs}
            }
            impl std::cmp::PartialEq<$SelfT> for $ActualT{
                fn eq(&self,rhs:&$SelfT)->bool{*self==rhs.value()}
            }
            impl std::cmp::PartialOrd for $SelfT{
                fn partial_cmp(&self,rhs:&Self)->Option<std::cmp::Ordering>{self.value().partial_cmp(&rhs.value())}
                fn lt(&self,rhs:&Self)->bool{self.value()<rhs.value()}
                fn le(&self,rhs:&Self)->bool{self.value()<=rhs.value()}
                fn gt(&self,rhs:&Self)->bool{self.value()>rhs.value()}
                fn ge(&self,rhs:&Self)->bool{self.value()>=rhs.value()}
            }
            impl std::cmp::PartialOrd<$ActualT> for $SelfT{
                fn partial_cmp(&self,rhs:&$ActualT)->Option<std::cmp::Ordering>{self.value().partial_cmp(rhs)}
                fn lt(&self,rhs:&$ActualT)->bool{self.value()<*rhs}
                fn le(&self,rhs:&$ActualT)->bool{self.value()<=*rhs}
                fn gt(&self,rhs:&$ActualT)->bool{self.value()>*rhs}
                fn ge(&self,rhs:&$ActualT)->bool{self.value()>=*rhs}
            }
            impl std::cmp::PartialOrd<$SelfT> for $ActualT{
                fn partial_cmp(&self,rhs:&$SelfT)->Option<std::cmp::Ordering>{self.partial_cmp(&rhs.value())}
                fn lt(&self,rhs:&$SelfT)->bool{*self<rhs.value()}
                fn le(&self,rhs:&$SelfT)->bool{*self<=rhs.value()}
                fn gt(&self,rhs:&$SelfT)->bool{*self>rhs.value()}
                fn ge(&self,rhs:&$SelfT)->bool{*self>=rhs.value()}
            }
            impl std::cmp::Ord for $SelfT{
                fn cmp(&self,rhs:&Self)->std::cmp::Ordering{self.value().cmp(&rhs.value())}
            }
        }
        pub use $OET::{$SelfT,$SelfE};
    }
}
Oint_impl!{Oi8,Ei8,i8,oei8}
Oint_impl!{Oi16,Ei16,i16,oei16}
Oint_impl!{Oi32,Ei32,i32,oei32}
Oint_impl!{Oi64,Ei64,i64,oei64}
Oint_impl!{Oi128,Ei128,i128,oei128}
Oint_impl!{Oisize,Eisize,isize,oeisize}
Oint_impl!{Ou8,Eu8,u8,oeu8}
Oint_impl!{Ou16,Eu16,u16,oeu16}
Oint_impl!{Ou32,Eu32,u32,oeu32}
Oint_impl!{Ou64,Eu64,u64,oeu64}
Oint_impl!{Ou128,Eu128,u128,oeu128}
Oint_impl!{Ousize,Eusize,usize,oeusize}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        assert!(&Ou128::from_raw((1,Eu128::SS(1,2))) + Ou8::new(4).value() as u128==7);
        assert!(0u8-Ou8::new(4).value()>0);
        assert_eq!({cai!{
            let _a=1;               // a normal statement
            let mut a:i32 :=0;      // Custom Initialize (let mut a:i32 =CustomInitialize::custom_initialize())
            a+=1;                   // stmt
            assert_eq!(1,a);
            for i in 0i32..1{
                let j:i32 :=3;
                assert_eq!(2,j-a+i)
            }
            a~2;                    // Custom Assign (bind to `~`)
            if a==2{
                let j:i32 :=1;
                assert_eq!(3,a+j)
            }
            a.custom_assign(6i32);  // expr with no ending semicolon
            if a==1i32{
            }else{
                let j:i32 :=2;
                assert_eq!(4,a-j)
            }
            while a!=1{
                a:=1;               // := could be used without let clause.
                assert_eq!(a,1)
            }
            loop{
                assert_eq!(6,a+5);
                break {
                    let b:i32 := 7 ;
                    assert_eq!(7,b) ;
                    b
                }
            }
            assert_eq!(8,a+7);
            3 //if you really want cai! return something to an expression, using {cai!{...}}
               // fortunately, fn _()->_ already have the form fn _()->_  {cai!{...}}
        }},3);
    }
}
//*
fn main(){
    let mut a=Oi32::new(0);
    let mut b=Oi32::new_cssc(0);
    for _ in 0..100{
        eprintln!("{} {}:{:?}",a.value(),b.value(),a);
        a+=1;
        b+=a.clone();
        b-=&a;
        b.assign(&b+&a)
    }
}// */
