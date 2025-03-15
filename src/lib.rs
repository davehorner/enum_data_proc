use std::str::FromStr;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Attribute, Ident};
use quote::quote;

#[proc_macro_derive(BitOp)]
pub fn derive_impl_BitOp(item:TokenStream)->TokenStream{
    let input=parse_macro_input!(item as DeriveInput);

    let check_is_enum=|data:&Data|if let Data::Enum(_x)=data{true}else{false};

    if !check_is_enum(&input.data){
        panic!("#[derive(BitOp)] is only defined for enums")
    }

    fn repr_attr(attrs:&Vec<Attribute>)->Option<&Attribute>{attrs.iter().filter(|x|x.path().is_ident("repr")).next()}

    let repr=repr_attr(&input.attrs);

    if repr.is_none(){
        panic!("#[derive(BitOp)] not find repr attribute")
    };

    let repr_d:Ident=repr.unwrap().parse_args::<Ident>().expect("failed to parse repr attribute");

    let check_repr_data=|repr:&Ident|
        match repr.to_string().as_str(){
            "u8"|"u16"|"u32"|"u64"|"u128"|"usize"
                |"i8"|"i16"|"i32"|"i64"|"i128"|"isize"=>true,
            _=>false
        };

    if !check_repr_data(&repr_d){
        panic!("invalid repr attribute for #[derive(BitOp)]")
    }

    let enum_name_=input.ident.to_string();
    let int_type_=repr_d.to_string();

    let enum_name=enum_name_.as_str();
    let int_type=int_type_.as_str();
    
    let not_oper=String::from(r#"
        impl std::ops::Not for @enum_name {
            type Output = @int_type;
            
            #[inline]
            fn not(self) -> Self::Output { !(self as @int_type) }
        }
    "#);
    let not_oper=not_oper
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type);
    let bit_oper_a=String::from(r#"
        impl std::ops::@trait_name<@int_type> for @enum_name {
            type Output = @int_type;

           #[inline]
            fn @trait_func(self, rhs: @int_type) -> Self::Output { (self as @int_type) @op rhs }
        }
        impl std::ops::@trait_name<@enum_name> for @enum_name {
            type Output = @int_type;

           #[inline]
            fn @trait_func(self, rhs: @enum_name) -> Self::Output { (self as @int_type) @op (rhs as @int_type) }
        }
        impl std::ops::@trait_name<@enum_name> for @int_type {
            type Output = @int_type;

            #[inline]
            fn @trait_func(self, rhs: @enum_name) -> Self::Output { self @op (rhs as @int_type) }
        }
    "#);
    let and_oper=bit_oper_a
        .replace("@trait_name", "BitAnd")
        .replace("@trait_func", "bitand")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "&");
    let or_oper=bit_oper_a
        .replace("@trait_name", "BitOr")
        .replace("@trait_func", "bitor")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "|");
    let xor_oper=bit_oper_a
        .replace("@trait_name", "BitXor")
        .replace("@trait_func", "bitxor")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "^");
    let bit_oper_b=String::from(r#"
        impl std::ops::@trait_name<@shift_type> for @enum_name {
            type Output = @int_type;
    
           #[inline]
            fn @trait_func(self, rhs: @shift_type) -> Self::Output { (self as @int_type) @op rhs }
        }
    "#);
    let shl_oper:String = ["u8","u16","u32","u64","u128","usize","i8","i16","i32","i64","i128","isize"]
        .iter().map(|st|bit_oper_b
            .replace("@trait_name", "Shl")
            .replace("@trait_func", "shl")
            .replace("@enum_name", enum_name)
            .replace("@int_type", int_type)
            .replace("@shift_type", st)
            .replace("@op", "<<")).collect();
    
    let shr_oper:String = ["u8","u16","u32","u64","u128","usize","i8","i16","i32","i64","i128","isize"]
        .iter().map(|st|bit_oper_b
            .replace("@trait_name", "Shr")
            .replace("@trait_func", "shr")
            .replace("@enum_name", enum_name)
            .replace("@int_type", int_type)
            .replace("@shift_type", st)
            .replace("@op", ">>")).collect();
    
    let mut r=String::new();
    r.push_str(not_oper.as_str());
    r.push_str(and_oper.as_str());
    r.push_str(or_oper.as_str());
    r.push_str(xor_oper.as_str());
    r.push_str(shl_oper.as_str());
    r.push_str(shr_oper.as_str());
    TokenStream::from_str(r.as_str()).expect("?")
}

#[proc_macro_derive(ArithOp)]
pub fn derive_impl_ArithOp(item:TokenStream)->TokenStream{
    let input=parse_macro_input!(item as DeriveInput);

    let check_is_enum=|data:&Data|if let Data::Enum(_x)=data{true}else{false};

    if !check_is_enum(&input.data){
        panic!("#[derive(ArithOp)] is only defined for enums")
    }

    fn repr_attr(attrs:&Vec<Attribute>)->Option<&Attribute>{attrs.iter().filter(|x|x.path().is_ident("repr")).next()}

    let repr=repr_attr(&input.attrs);

    if repr.is_none(){
        panic!("#[derive(ArithOp)] not find repr attribute")
    };

    let repr_d:Ident=repr.unwrap().parse_args::<Ident>().expect("failed to parse repr attribute");

    let check_repr_data=|repr:&Ident|
        match repr.to_string().as_str(){
            "u8"|"u16"|"u32"|"u64"|"u128"|"usize"
                |"i8"|"i16"|"i32"|"i64"|"i128"|"isize"=>true,
            _=>false
        };

    if !check_repr_data(&repr_d){
        panic!("invalid repr attribute for #[derive(ArithOp)]")
    }

    let enum_name_=input.ident.to_string();
    let int_type_=repr_d.to_string();

    let enum_name=enum_name_.as_str();
    let int_type=int_type_.as_str();

    let arith_oper=String::from(r#"
        impl std::ops::@trait_name<@int_type> for @enum_name {
            type Output = @int_type;

           #[inline]
            fn @trait_func(self, rhs: @int_type) -> Self::Output { (self as @int_type) @op rhs }
        }
        impl std::ops::@trait_name<@enum_name> for @enum_name {
            type Output = @int_type;

           #[inline]
            fn @trait_func(self, rhs: @enum_name) -> Self::Output { (self as @int_type) @op (rhs as @int_type) }
        }
        impl std::ops::@trait_name<@enum_name> for @int_type {
            type Output = @int_type;

            #[inline]
            fn @trait_func(self, rhs: @enum_name) -> Self::Output { self @op (rhs as @int_type) }
        }
    "#);

    let add_oper=arith_oper
        .replace("@trait_name", "Add")
        .replace("@trait_func", "add")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "+");
    let sub_oper=arith_oper
        .replace("@trait_name", "Sub")
        .replace("@trait_func", "sub")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "-");
    let mul_oper=arith_oper
        .replace("@trait_name", "Mul")
        .replace("@trait_func", "mul")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "*");
    
    let div_oper=arith_oper
        .replace("@trait_name", "Div")
        .replace("@trait_func", "div")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "/");
    let rem_oper=arith_oper
        .replace("@trait_name", "Rem")
        .replace("@trait_func", "rem")
        .replace("@enum_name", enum_name)
        .replace("@int_type", int_type)
        .replace("@op", "%");

    let mut r=String::new();
    r.push_str(add_oper.as_str());
    r.push_str(sub_oper.as_str());
    r.push_str(div_oper.as_str());
    r.push_str(mul_oper.as_str());
    r.push_str(rem_oper.as_str());
    TokenStream::from_str(r.as_str()).expect("?")
}
