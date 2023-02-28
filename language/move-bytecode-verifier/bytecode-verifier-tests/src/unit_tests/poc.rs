use invalid_mutations::bounds::{
    ApplyCodeUnitBoundsContext, ApplyOutOfBoundsContext, CodeUnitBoundsMutation,
    OutOfBoundsMutation,
};
use move_binary_format::{
    check_bounds::BoundsChecker, file_format::*, file_format_common,
    proptest_types::CompiledModuleStrategyGen,
};
use move_core_types::{
    account_address::AccountAddress, identifier::Identifier, vm_status::StatusCode,
};
use proptest::{collection::vec, prelude::*};
use move_binary_format::file_format::SignatureToken::U64;

use move_binary_format::file_format::SignatureToken::Reference;

use move_binary_format::{
    binary_views::BinaryIndexedView,
    errors::{
        bounds_error, offset_out_of_bounds as offset_out_of_bounds_error, verification_error,
        PartialVMError, PartialVMResult,
    },
    file_format::{
        AbilitySet, Bytecode, CodeOffset, CodeUnit, CompiledModule, CompiledScript, Constant,
        FieldHandle, FieldInstantiation, FunctionDefinition, FunctionDefinitionIndex,
        FunctionHandle, FunctionInstantiation, LocalIndex, ModuleHandle, Signature, SignatureToken,
        StructDefInstantiation, StructDefinition, StructFieldInformation, StructHandle, TableIndex,
    },
    internals::ModuleIndex,
    IndexKind,
};


use move_core_types::metadata::Metadata;
use move_binary_format::file_format::Bytecode::*;

use move_bytecode_verifier::{verify_script, verify_module};

use std::fs::File;
use std::io::Read;
use std::io::Write;
#[test]
fn poc_1() {
let sigs0 = (0..179)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..242)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..37)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..209)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..21)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![Branch(0), MoveLoc(128), LdFalse, CastU128, BrFalse(6), ReadRef, CastU32, CastU256, BrTrue(1), LdU16(64095), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_2() {
let sigs0 = (0..155)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..245)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..171)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..138)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..1)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![Branch(0), ImmBorrowLoc(122), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_3() {
let sigs0 = (0..78)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, CastU256, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_4() {
let sigs0 = (0..144)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..40)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU16(2542), LdFalse, ReadRef, Nop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_5() {
let sigs0 = (0..249)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..1)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU32(2242288821), LdTrue, LdU128(260107455918736101942863101762398901367), Xor, CastU32, StLoc(75), LdU8(24), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_6() {
let sigs0 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_7() {
let sigs0 = (0..255)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..30)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..180)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..196)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..109)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..53)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..221)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..248)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..175)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(5),code: {vec![LdU16(58177), CastU16, BrFalse(5), LdU256(move_core_types::u256::U256::from(235614548965005283575801269078222562719_u128)), BrTrue(4), MoveLoc(14), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_8() {
let sigs0 = (0..159)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..177)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..36)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..24)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..185)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..75)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..75)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdTrue, ImmBorrowLoc(106), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_9() {
let sigs0 = (0..174)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..3)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..136)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..201)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..201)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..229)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..36)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![LdU128(116970863421515816436444317551097253048), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_10() {
let sigs0 = (0..240)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..36)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..110)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU64(13315890527966523382), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_11() {
let sigs0 = (0..111)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU8(235), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_12() {
let sigs0 = (0..89)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![Branch(0), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_13() {
let sigs0 = (0..115)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..136)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MoveLoc(184), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_14() {
let sigs0 = (0..222)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..5)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![CopyLoc(34), CastU16, CastU128, LdTrue, Add, ReadRef, LdFalse, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_15() {
let sigs0 = (0..173)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..254)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..93)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..240)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..241)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..140)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..0)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU32(2025901886), CastU32, Nop, StLoc(36), LdU32(2557615740), ImmBorrowLoc(62), Ge, LdTrue, LdU16(19314), Le, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_16() {
let sigs0 = (0..215)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, CastU64, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_17() {
let sigs0 = (0..107)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..172)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..197)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..50)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..249)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..126)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..109)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..180)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..214)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![MoveLoc(18), LdFalse, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_18() {
let sigs0 = (0..138)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..217)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..198)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..135)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..41)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..77)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..96)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![LdU64(15035127814467492436), MutBorrowLoc(132), WriteRef, LdU256(move_core_types::u256::U256::from(102590333630012724676940360500024964118_u128)), LdTrue, Div, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_19() {
let sigs0 = (0..51)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..242)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..224)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..122)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..147)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..84)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MutBorrowLoc(86), LdU256(move_core_types::u256::U256::from(2876052851731126248771449597137253039_u128)), LdU64(15816416252232126765), Gt, LdU32(913979276), Nop, Le, CopyLoc(13), Xor, CastU16, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_20() {
let sigs0 = (0..100)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..242)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..121)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..80)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..180)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..29)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..183)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..170)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..212)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(8),code: {vec![LdTrue, CastU128, Nop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_21() {
let sigs0 = (0..187)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU8(223), LdU32(4105703910), Shr, StLoc(194), LdU64(7814406433316165190), CastU64, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_22() {
let sigs0 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..155)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..61)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..214)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..22)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..12)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..145)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..115)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![CopyLoc(18), ReadRef, CastU64, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_23() {
let sigs0 = (0..45)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..141)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..172)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..173)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..71)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdU256(move_core_types::u256::U256::from(184184729221371909934388051245585900898_u128)), CastU16, CopyLoc(218), Nop, LdU64(12757172743022759383), Mul, Neq, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_24() {
let sigs0 = (0..53)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..134)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..169)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..9)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..97)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..62)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..38)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(5),code: {vec![LdU32(2291520044), BrFalse(1), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_25() {
let sigs0 = (0..70)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..85)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..112)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..77)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..67)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..212)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..162)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..140)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU32(3972413148), ReadRef, BrTrue(0), CopyLoc(46), LdU16(6587), ImmBorrowLoc(134), CastU8, Shr, Or, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_26() {
let sigs0 = (0..67)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![Nop, LdU256(move_core_types::u256::U256::from(64997210160906854776541510106062760019_u128)), Abort, LdU32(572425056), CastU128, LdTrue, Nop, Mod, CastU16, LdU8(28), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_27() {
let sigs0 = (0..199)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..101)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..42)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, Nop, Branch(0), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_28() {
let sigs0 = (0..41)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..188)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..225)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..131)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..120)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..33)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..58)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![Nop, LdU64(15449386696545084339), LdU8(56), Gt, LdU64(12135163101494094919), CopyLoc(88), Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_29() {
let sigs0 = (0..35)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..80)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..123)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..111)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..219)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..74)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![MoveLoc(170), Nop, Nop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_30() {
let sigs0 = (0..79)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..3)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..205)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..245)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..208)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..113)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..112)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..43)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..102)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..105)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(5),code: {vec![ImmBorrowLoc(206), CastU128, CastU16, CastU64, LdTrue, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_31() {
let sigs0 = (0..163)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..248)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..143)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..82)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..166)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..43)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![LdFalse, Nop, MutBorrowLoc(99), Xor, Not, LdU32(2122898838), LdTrue, LdU128(158930189632297327413168711034215861568), LdU128(262424274609440755802865695258305779123), CastU64, Pop, Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_32() {
let sigs0 = (0..110)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, CastU32, LdU256(move_core_types::u256::U256::from(173141017355967656389413700952746978955_u128)), BrTrue(2), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_33() {
let sigs0 = (0..230)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU256(move_core_types::u256::U256::from(17201395504669958252236172714493665483_u128)), LdU16(20918), MutBorrowLoc(160), Neq, Branch(2), LdU256(move_core_types::u256::U256::from(9675014355309548629906382475285244833_u128)), LdU128(96794884258638378760912425813976966486), Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_34() {
let sigs0 = (0..222)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..106)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..211)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..173)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..164)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..230)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(5),code: {vec![Branch(3), ImmBorrowLoc(147), Abort, LdU32(3935563584), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_35() {
let sigs0 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..119)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..217)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..84)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..122)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..68)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![Nop, LdU256(move_core_types::u256::U256::from(59367847909069294468980609247259953816_u128)), CastU16, MutBorrowLoc(178), LdFalse, LdU128(87675972770068824125138665536548454541), LdFalse, MoveLoc(7), CastU256, Lt, Pop, Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_36() {
let sigs0 = (0..87)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..135)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![ImmBorrowLoc(1), StLoc(42), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_37() {
let sigs0 = (0..247)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..150)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..244)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..246)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..195)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU256(move_core_types::u256::U256::from(20058589690632434869061841170473843207_u128)), MutBorrowLoc(90), Mod, Not, BrFalse(4), LdU8(212), Branch(5), Abort, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_38() {
let sigs0 = (0..142)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..9)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..152)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..217)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..160)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..98)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdTrue, CastU16, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_39() {
let sigs0 = (0..247)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..171)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..141)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..4)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU16(3944), Nop, LdU128(162321302152652268986519067668223401285), Eq, LdU64(5137565944245278521), CastU128, LdU8(248), Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_40() {
let sigs0 = (0..217)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..174)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..152)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..68)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..25)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdU16(10305), CastU128, LdU8(232), MoveLoc(57), Le, Abort, Branch(3), ImmBorrowLoc(97), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_41() {
let sigs0 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MutBorrowLoc(131), LdFalse, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_42() {
let sigs0 = (0..42)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..187)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..47)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..95)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdTrue, LdTrue, LdFalse, ImmBorrowLoc(69), Shr, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_43() {
let sigs0 = (0..82)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..24)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..209)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..135)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..24)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..104)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..131)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..56)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(8),code: {vec![ImmBorrowLoc(28), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_44() {
let sigs0 = (0..61)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..218)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..121)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![Nop, LdFalse, LdTrue, BrFalse(0), FreezeRef, LdU64(197965364105407747), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_45() {
let sigs0 = (0..104)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..102)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..191)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..151)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..205)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..174)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..35)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![MutBorrowLoc(29), CastU8, MoveLoc(181), Or, BrTrue(5), Nop, LdU16(62725), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_46() {
let sigs0 = (0..34)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MoveLoc(0), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_47() {
let sigs0 = (0..172)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..250)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..228)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![ImmBorrowLoc(146), ImmBorrowLoc(66), FreezeRef, Not, LdU8(42), LdU16(8721), BrTrue(9), LdU128(326149224684643910987387127872648027732), BitAnd, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_48() {
let sigs0 = (0..164)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..128)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..226)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..81)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..89)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..100)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..128)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![Nop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_49() {
let sigs0 = (0..72)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..63)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..134)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..93)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..202)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..59)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..99)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..151)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..119)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(6),code: {vec![MoveLoc(163), CastU32, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_50() {
let sigs0 = (0..45)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..6)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..153)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..133)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..247)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..238)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..80)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU128(289875008513203506682949495191736197927), FreezeRef, ImmBorrowLoc(60), LdU8(173), Abort, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_51() {
let sigs0 = (0..234)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..83)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..123)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![ImmBorrowLoc(74), LdU8(103), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_52() {
let sigs0 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..166)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..128)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..153)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..115)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..120)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..185)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..102)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..210)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..32)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(8),code: {vec![LdU64(6599207226229614311), Branch(0), LdTrue, MutBorrowLoc(114), StLoc(219), BrTrue(6), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_53() {
let sigs0 = (0..79)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..100)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..130)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..169)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdTrue, Branch(3), Branch(0), LdU256(move_core_types::u256::U256::from(247456269715104763712064578371402903648_u128)), Add, CastU64, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_54() {
let sigs0 = (0..41)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..24)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..111)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..71)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..56)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..110)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..75)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..70)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..229)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(6),code: {vec![MoveLoc(59), LdU128(133974965189345935237755769294571462236), LdU16(9190), Add, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_55() {
let sigs0 = (0..135)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..60)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..211)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..199)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..208)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..43)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![MutBorrowLoc(71), LdU8(254), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_56() {
let sigs0 = (0..253)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..94)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..203)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..148)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..239)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..251)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..147)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..152)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..79)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU256(move_core_types::u256::U256::from(258923642737997593930534268314818382529_u128)), LdFalse, Shr, LdU16(52756), Sub, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_57() {
let sigs0 = (0..131)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..223)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..22)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..107)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..144)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..139)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..130)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(4),code: {vec![LdU8(191), Not, LdU64(1560100429522940915), Gt, Nop, Abort, MutBorrowLoc(218), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_58() {
let sigs0 = (0..250)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..105)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..167)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..237)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..11)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdFalse, CopyLoc(241), CastU16, Gt, ImmBorrowLoc(236), WriteRef, LdFalse, Abort, CopyLoc(68), StLoc(197), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_59() {
let sigs0 = (0..94)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..51)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..115)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..12)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..200)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..168)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..34)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![Branch(2), MutBorrowLoc(5), LdU256(move_core_types::u256::U256::from(213615905150293211795547438168853435102_u128)), LdU256(move_core_types::u256::U256::from(47832626022045462522485223599894086756_u128)), Add, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_60() {
let sigs0 = (0..98)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..249)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..161)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..156)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..5)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![CopyLoc(7), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_61() {
let sigs0 = (0..62)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..141)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..143)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..222)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..10)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU64(16124081607859247919), LdU64(4849343884416851366), Ge, BrTrue(2), Nop, LdU16(44327), CopyLoc(130), Not, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_62() {
let sigs0 = (0..6)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..247)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, LdU8(165), Abort, CastU16, MutBorrowLoc(97), And, LdU64(16782270938625549498), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_63() {
let sigs0 = (0..173)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..40)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..127)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..117)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..38)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..84)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdU8(151), LdU8(55), Shr, LdU64(11865072206227100049), CastU128, Lt, LdU32(86177699), LdU128(151285734184555012404596293606043086140), CopyLoc(80), Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_64() {
let sigs0 = (0..228)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU64(2576251288114435775), CastU128, CastU64, MutBorrowLoc(86), Nop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_65() {
let sigs0 = (0..19)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..245)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..143)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..38)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![CopyLoc(252), BrTrue(2), LdU128(327443621871826865448804403633018617694), LdFalse, Shl, LdU256(move_core_types::u256::U256::from(183125516262569953574139845064201588193_u128)), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_66() {
let sigs0 = (0..167)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..155)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..48)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..69)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..104)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU8(163), LdU128(330729011634581299552139920287900511794), Lt, ReadRef, LdU32(2188386342), ImmBorrowLoc(165), Eq, WriteRef, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_67() {
let sigs0 = (0..121)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU128(105298406485884392311540023949395098521), Not, LdU16(22290), LdU128(49868957778564688808240792749647126680), Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_68() {
let sigs0 = (0..225)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..13)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![ImmBorrowLoc(204), CastU256, Abort, LdU128(74786729740682808318399749814476393193), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_69() {
let sigs0 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![CopyLoc(47), MoveLoc(30), Gt, FreezeRef, MoveLoc(47), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_70() {
let sigs0 = (0..204)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..110)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..251)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..199)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..85)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..171)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..238)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..224)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs9 = (0..16)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8), Signature(sigs9)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU64(3055222362743179846), CastU128, StLoc(45), MutBorrowLoc(32), ImmBorrowLoc(145), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(9), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_71() {
let sigs0 = (0..197)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..218)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..74)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..208)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..30)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..18)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdU32(1775094829), BrFalse(4), LdU128(193652191005342178251398151302189942628), CastU16, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_72() {
let sigs0 = (0..235)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MutBorrowLoc(135), CastU32, BrTrue(7), LdU8(231), LdU64(13221274177568627299), ImmBorrowLoc(162), LdTrue, CastU128, Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_73() {
let sigs0 = (0..215)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU32(1696507860), CastU32, Nop, MoveLoc(8), CastU32, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_74() {
let sigs0 = (0..172)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..27)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..109)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..2)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![ImmBorrowLoc(162), Abort, ImmBorrowLoc(149), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_75() {
let sigs0 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..54)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..37)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..50)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MutBorrowLoc(7), StLoc(32), LdU256(move_core_types::u256::U256::from(261813515940894229551126414340738896639_u128)), LdU128(33384752076010891550614729346080554392), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_76() {
let sigs0 = (0..221)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..149)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..8)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..119)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..183)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..123)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..169)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..144)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..6)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdU256(move_core_types::u256::U256::from(265027840115070988742221438879635378604_u128)), CastU64, LdU16(52910), MoveLoc(12), Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_77() {
let sigs0 = (0..11)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..64)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..73)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..63)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..201)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..230)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..57)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..2)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(7),code: {vec![MutBorrowLoc(17), Nop, Not, Abort, CopyLoc(41), CastU128, FreezeRef, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_78() {
let sigs0 = (0..180)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..140)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..128)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..157)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..20)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU16(28080), CastU16, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_79() {
let sigs0 = (0..3)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..168)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..141)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..23)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..61)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU32(3645973462), Nop, LdFalse, ReadRef, Eq, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_80() {
let sigs0 = (0..146)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdU128(326451638698047920612002134795647996641), LdU32(1772150934), LdU16(32259), Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(0), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_81() {
let sigs0 = (0..4)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..73)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..146)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..76)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..43)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..172)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..3)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdFalse, MutBorrowLoc(81), Branch(3), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_82() {
let sigs0 = (0..255)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..58)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..104)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..60)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..52)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..61)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..51)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..167)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(7),code: {vec![LdU64(15192000908319810698), Not, MutBorrowLoc(200), BitAnd, CastU8, LdTrue, BrTrue(1), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_83() {
let sigs0 = (0..60)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..71)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..84)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![MutBorrowLoc(112), BrTrue(1), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_84() {
let sigs0 = (0..199)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..218)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..222)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..97)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..101)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![CopyLoc(135), LdU256(move_core_types::u256::U256::from(43346038605203136141593749032614698973_u128)), FreezeRef, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_85() {
let sigs0 = (0..71)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..90)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..241)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..126)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..108)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..122)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MoveLoc(126), LdU32(2790274031), LdU64(1912087190370743012), MoveLoc(66), ReadRef, And, Mul, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_86() {
let sigs0 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..137)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..171)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..157)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..148)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..34)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..251)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..170)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(5),code: {vec![LdU16(19375), MutBorrowLoc(160), LdU32(3171919847), Shl, WriteRef, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_87() {
let sigs0 = (0..63)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..54)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![LdFalse, CastU32, MoveLoc(14), Xor, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_88() {
let sigs0 = (0..150)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..230)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..169)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..242)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..73)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..219)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..113)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..44)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs8 = (0..31)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7), Signature(sigs8)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![MoveLoc(140), LdU32(2018040345), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(8), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_89() {
let sigs0 = (0..131)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..247)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..153)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..107)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..57)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdFalse, LdU32(1798347050), CastU16, CastU32, LdU64(3505073928849127729), Xor, LdU8(249), Le, Abort, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_90() {
let sigs0 = (0..138)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..98)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(0),code: {vec![ImmBorrowLoc(68), CastU8, ReadRef, CopyLoc(160), WriteRef, LdU256(move_core_types::u256::U256::from(21604583504004803787158795102455899047_u128)), Not, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(1), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_91() {
let sigs0 = (0..47)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..52)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..15)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU256(move_core_types::u256::U256::from(116774910710197590809076994512141667539_u128)), CastU256, Not, ImmBorrowLoc(42), Mod, CastU64, Branch(5), BrFalse(4), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_92() {
let sigs0 = (0..48)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..235)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..60)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..57)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..1)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdTrue, LdU16(49068), Eq, CastU8, Nop, LdU16(63742), MutBorrowLoc(1), Lt, Ge, LdU32(2895168033), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_93() {
let sigs0 = (0..239)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..96)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..254)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..100)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU128(51174236386013362124707048040991240229), CopyLoc(185), Or, LdU256(move_core_types::u256::U256::from(257091886221946751387722965704737434031_u128)), Ge, Abort, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_94() {
let sigs0 = (0..102)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..92)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..38)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..145)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![Nop, CopyLoc(59), ReadRef, LdU32(1957458546), CopyLoc(16), Xor, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(3), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_95() {
let sigs0 = (0..210)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..7)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..56)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..85)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..166)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![MutBorrowLoc(35), CastU128, StLoc(234), LdU256(move_core_types::u256::U256::from(147173760645462159033293799236801482437_u128)), StLoc(28), Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(4), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_96() {
let sigs0 = (0..210)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..144)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..40)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..124)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..245)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..200)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..62)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![LdFalse, LdU32(1941774701), Shl, LdU128(190948717098681248825641305984157817481), BrTrue(0), BrTrue(7), Branch(2), LdU256(move_core_types::u256::U256::from(174103154736832524184063514203227441308_u128)), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(6), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_97() {
let sigs0 = (0..20)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..221)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..251)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..49)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..97)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..119)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs6 = (0..127)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs7 = (0..117)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5), Signature(sigs6), Signature(sigs7)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU16(14033), LdFalse, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(7), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_98() {
let sigs0 = (0..203)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..221)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..0)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(1),code: {vec![LdU128(154477313846965349282212133184885615683), Abort, LdU16(27055), LdU16(15538), LdU8(160), LdU8(196), Pop, Pop, Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(2), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_99() {
let sigs0 = (0..163)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..188)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..229)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..91)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..61)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..20)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(2),code: {vec![Nop, LdU8(1), BrFalse(1), LdU8(220), Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}

#[test]
fn poc_100() {
let sigs0 = (0..117)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs1 = (0..181)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs2 = (0..66)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs3 = (0..254)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs4 = (0..154)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let sigs5 = (0..1)
        .map(|_| Reference(Box::new(U64)))
        .collect::<Vec<_>>();
let script = CompiledScript {
version: 5, 
module_handles: vec![], 
struct_handles: vec![], 
function_handles: vec![], 
function_instantiations: vec![], 
signatures: vec![Signature(sigs0), Signature(sigs1), Signature(sigs2), Signature(sigs3), Signature(sigs4), Signature(sigs5)], 
identifiers: vec![], 
address_identifiers: vec![], 
constant_pool: vec![], 
metadata: vec![Metadata {
            key: vec![],
            value: vec![],
        }], 
code: CodeUnit {locals: SignatureIndex(3),code: {vec![LdU64(10933032376710418374), ImmBorrowLoc(2), And, LdU16(13730), Pop, Pop, Ret]},}, 
type_parameters: vec![], 
parameters: SignatureIndex(5), 
};
let res = verify_script(&script);
res.unwrap();
}
