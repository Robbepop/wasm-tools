/// A helper macro to conveniently iterate over all opcodes recognized by this
/// crate. This can be used to work with either the [`SimdOperator`] enumeration or
/// the [`VisitSimdOperator`] trait if your use case uniformly handles all operators
/// the same way.
///
/// The list of specializable Wasm proposals is as follows:
///
/// - `@simd`: [Wasm `simd` proposal]
/// - `@relaxed_simd`: [Wasm `relaxed-simd` proposal]
///
/// For more information about the structure and use of this macro please
/// refer to the documentation of the [`for_each_operator`] macro.
///
/// [Wasm `simd` proposal]:
/// https://github.com/webassembly/simd
///
/// [Wasm `relaxed-simd` proposal]:
/// https://github.com/WebAssembly/relaxed-simd
///
/// [`SimdOperator`]: crate::SimdOperator
/// [`VisitSimdOperator`]: crate::VisitSimdOperator
#[macro_export]
macro_rules! for_each_simd_operator {
    ($mac:ident) => {
        $mac! {
            // 0xFD operators
            // 128-bit SIMD
            // - https://github.com/webassembly/simd
            // - https://webassembly.github.io/simd/core/binary/instructions.html
            @simd V128Load { memarg: $crate::MemArg } => visit_v128_load (load v128)
            @simd V128Load8x8S { memarg: $crate::MemArg } => visit_v128_load8x8_s (load v128)
            @simd V128Load8x8U { memarg: $crate::MemArg } => visit_v128_load8x8_u (load v128)
            @simd V128Load16x4S { memarg: $crate::MemArg } => visit_v128_load16x4_s (load v128)
            @simd V128Load16x4U { memarg: $crate::MemArg } => visit_v128_load16x4_u (load v128)
            @simd V128Load32x2S { memarg: $crate::MemArg } => visit_v128_load32x2_s (load v128)
            @simd V128Load32x2U { memarg: $crate::MemArg } => visit_v128_load32x2_u (load v128)
            @simd V128Load8Splat { memarg: $crate::MemArg } => visit_v128_load8_splat (load v128)
            @simd V128Load16Splat { memarg: $crate::MemArg } => visit_v128_load16_splat (load v128)
            @simd V128Load32Splat { memarg: $crate::MemArg } => visit_v128_load32_splat (load v128)
            @simd V128Load64Splat { memarg: $crate::MemArg } => visit_v128_load64_splat (load v128)
            @simd V128Load32Zero { memarg: $crate::MemArg } => visit_v128_load32_zero (load v128)
            @simd V128Load64Zero { memarg: $crate::MemArg } => visit_v128_load64_zero (load v128)
            @simd V128Store { memarg: $crate::MemArg } => visit_v128_store (store v128)
            @simd V128Load8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load8_lane (load lane 16)
            @simd V128Load16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load16_lane (load lane 8)
            @simd V128Load32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load32_lane (load lane 4)
            @simd V128Load64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load64_lane (load lane 2)
            @simd V128Store8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store8_lane (store lane 16)
            @simd V128Store16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store16_lane (store lane 8)
            @simd V128Store32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store32_lane (store lane 4)
            @simd V128Store64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store64_lane (store lane 2)
            @simd V128Const { value: $crate::V128 } => visit_v128_const (push v128)
            @simd I8x16Shuffle { lanes: [u8; 16] } => visit_i8x16_shuffle (arity 2 -> 1)
            @simd I8x16ExtractLaneS { lane: u8 } => visit_i8x16_extract_lane_s (extract i32 16)
            @simd I8x16ExtractLaneU { lane: u8 } => visit_i8x16_extract_lane_u (extract i32 16)
            @simd I8x16ReplaceLane { lane: u8 } => visit_i8x16_replace_lane (replace i32 16)
            @simd I16x8ExtractLaneS { lane: u8 } => visit_i16x8_extract_lane_s (extract i32 8)
            @simd I16x8ExtractLaneU { lane: u8 } => visit_i16x8_extract_lane_u (extract i32 8)
            @simd I16x8ReplaceLane { lane: u8 } => visit_i16x8_replace_lane (replace i32 8)
            @simd I32x4ExtractLane { lane: u8 } => visit_i32x4_extract_lane (extract i32 4)
            @simd I32x4ReplaceLane { lane: u8 } => visit_i32x4_replace_lane (replace i32 4)
            @simd I64x2ExtractLane { lane: u8 } => visit_i64x2_extract_lane (extract i64 2)
            @simd I64x2ReplaceLane { lane: u8 } => visit_i64x2_replace_lane (replace i64 2)
            @simd F32x4ExtractLane { lane: u8 } => visit_f32x4_extract_lane (extract f32 4)
            @simd F32x4ReplaceLane { lane: u8 } => visit_f32x4_replace_lane (replace f32 4)
            @simd F64x2ExtractLane { lane: u8 } => visit_f64x2_extract_lane (extract f64 2)
            @simd F64x2ReplaceLane { lane: u8 } => visit_f64x2_replace_lane (replace f64 2)
            @simd I8x16Swizzle => visit_i8x16_swizzle (binary v128)
            @simd I8x16Splat => visit_i8x16_splat (splat i32)
            @simd I16x8Splat => visit_i16x8_splat (splat i32)
            @simd I32x4Splat => visit_i32x4_splat (splat i32)
            @simd I64x2Splat => visit_i64x2_splat (splat i64)
            @simd F32x4Splat => visit_f32x4_splat (splat f32)
            @simd F64x2Splat => visit_f64x2_splat (splat f64)
            @simd I8x16Eq => visit_i8x16_eq (binary v128)
            @simd I8x16Ne => visit_i8x16_ne (binary v128)
            @simd I8x16LtS => visit_i8x16_lt_s (binary v128)
            @simd I8x16LtU => visit_i8x16_lt_u (binary v128)
            @simd I8x16GtS => visit_i8x16_gt_s (binary v128)
            @simd I8x16GtU => visit_i8x16_gt_u (binary v128)
            @simd I8x16LeS => visit_i8x16_le_s (binary v128)
            @simd I8x16LeU => visit_i8x16_le_u (binary v128)
            @simd I8x16GeS => visit_i8x16_ge_s (binary v128)
            @simd I8x16GeU => visit_i8x16_ge_u (binary v128)
            @simd I16x8Eq => visit_i16x8_eq (binary v128)
            @simd I16x8Ne => visit_i16x8_ne (binary v128)
            @simd I16x8LtS => visit_i16x8_lt_s (binary v128)
            @simd I16x8LtU => visit_i16x8_lt_u (binary v128)
            @simd I16x8GtS => visit_i16x8_gt_s (binary v128)
            @simd I16x8GtU => visit_i16x8_gt_u (binary v128)
            @simd I16x8LeS => visit_i16x8_le_s (binary v128)
            @simd I16x8LeU => visit_i16x8_le_u (binary v128)
            @simd I16x8GeS => visit_i16x8_ge_s (binary v128)
            @simd I16x8GeU => visit_i16x8_ge_u (binary v128)
            @simd I32x4Eq => visit_i32x4_eq (binary v128)
            @simd I32x4Ne => visit_i32x4_ne (binary v128)
            @simd I32x4LtS => visit_i32x4_lt_s (binary v128)
            @simd I32x4LtU => visit_i32x4_lt_u (binary v128)
            @simd I32x4GtS => visit_i32x4_gt_s (binary v128)
            @simd I32x4GtU => visit_i32x4_gt_u (binary v128)
            @simd I32x4LeS => visit_i32x4_le_s (binary v128)
            @simd I32x4LeU => visit_i32x4_le_u (binary v128)
            @simd I32x4GeS => visit_i32x4_ge_s (binary v128)
            @simd I32x4GeU => visit_i32x4_ge_u (binary v128)
            @simd I64x2Eq => visit_i64x2_eq (binary v128)
            @simd I64x2Ne => visit_i64x2_ne (binary v128)
            @simd I64x2LtS => visit_i64x2_lt_s (binary v128)
            @simd I64x2GtS => visit_i64x2_gt_s (binary v128)
            @simd I64x2LeS => visit_i64x2_le_s (binary v128)
            @simd I64x2GeS => visit_i64x2_ge_s (binary v128)
            @simd F32x4Eq => visit_f32x4_eq (binary v128f)
            @simd F32x4Ne => visit_f32x4_ne (binary v128f)
            @simd F32x4Lt => visit_f32x4_lt (binary v128f)
            @simd F32x4Gt => visit_f32x4_gt (binary v128f)
            @simd F32x4Le => visit_f32x4_le (binary v128f)
            @simd F32x4Ge => visit_f32x4_ge (binary v128f)
            @simd F64x2Eq => visit_f64x2_eq (binary v128f)
            @simd F64x2Ne => visit_f64x2_ne (binary v128f)
            @simd F64x2Lt => visit_f64x2_lt (binary v128f)
            @simd F64x2Gt => visit_f64x2_gt (binary v128f)
            @simd F64x2Le => visit_f64x2_le (binary v128f)
            @simd F64x2Ge => visit_f64x2_ge (binary v128f)
            @simd V128Not => visit_v128_not (unary v128)
            @simd V128And => visit_v128_and (binary v128)
            @simd V128AndNot => visit_v128_andnot (binary v128)
            @simd V128Or => visit_v128_or (binary v128)
            @simd V128Xor => visit_v128_xor (binary v128)
            @simd V128Bitselect => visit_v128_bitselect (ternary v128)
            @simd V128AnyTrue => visit_v128_any_true (test v128)
            @simd I8x16Abs => visit_i8x16_abs (unary v128)
            @simd I8x16Neg => visit_i8x16_neg (unary v128)
            @simd I8x16Popcnt => visit_i8x16_popcnt (unary v128)
            @simd I8x16AllTrue => visit_i8x16_all_true (test v128)
            @simd I8x16Bitmask => visit_i8x16_bitmask (test v128)
            @simd I8x16NarrowI16x8S => visit_i8x16_narrow_i16x8_s (binary v128)
            @simd I8x16NarrowI16x8U => visit_i8x16_narrow_i16x8_u (binary v128)
            @simd I8x16Shl => visit_i8x16_shl (shift v128)
            @simd I8x16ShrS => visit_i8x16_shr_s (shift v128)
            @simd I8x16ShrU => visit_i8x16_shr_u (shift v128)
            @simd I8x16Add => visit_i8x16_add (binary v128)
            @simd I8x16AddSatS => visit_i8x16_add_sat_s (binary v128)
            @simd I8x16AddSatU => visit_i8x16_add_sat_u (binary v128)
            @simd I8x16Sub => visit_i8x16_sub (binary v128)
            @simd I8x16SubSatS => visit_i8x16_sub_sat_s (binary v128)
            @simd I8x16SubSatU => visit_i8x16_sub_sat_u (binary v128)
            @simd I8x16MinS => visit_i8x16_min_s (binary v128)
            @simd I8x16MinU => visit_i8x16_min_u (binary v128)
            @simd I8x16MaxS => visit_i8x16_max_s (binary v128)
            @simd I8x16MaxU => visit_i8x16_max_u (binary v128)
            @simd I8x16AvgrU => visit_i8x16_avgr_u (binary v128)
            @simd I16x8ExtAddPairwiseI8x16S => visit_i16x8_extadd_pairwise_i8x16_s (unary v128)
            @simd I16x8ExtAddPairwiseI8x16U => visit_i16x8_extadd_pairwise_i8x16_u (unary v128)
            @simd I16x8Abs => visit_i16x8_abs (unary v128)
            @simd I16x8Neg => visit_i16x8_neg (unary v128)
            @simd I16x8Q15MulrSatS => visit_i16x8_q15mulr_sat_s (binary v128)
            @simd I16x8AllTrue => visit_i16x8_all_true (test v128)
            @simd I16x8Bitmask => visit_i16x8_bitmask (test v128)
            @simd I16x8NarrowI32x4S => visit_i16x8_narrow_i32x4_s (binary v128)
            @simd I16x8NarrowI32x4U => visit_i16x8_narrow_i32x4_u (binary v128)
            @simd I16x8ExtendLowI8x16S => visit_i16x8_extend_low_i8x16_s (unary v128)
            @simd I16x8ExtendHighI8x16S => visit_i16x8_extend_high_i8x16_s (unary v128)
            @simd I16x8ExtendLowI8x16U => visit_i16x8_extend_low_i8x16_u (unary v128)
            @simd I16x8ExtendHighI8x16U => visit_i16x8_extend_high_i8x16_u (unary v128)
            @simd I16x8Shl => visit_i16x8_shl (shift v128)
            @simd I16x8ShrS => visit_i16x8_shr_s (shift v128)
            @simd I16x8ShrU => visit_i16x8_shr_u (shift v128)
            @simd I16x8Add => visit_i16x8_add (binary v128)
            @simd I16x8AddSatS => visit_i16x8_add_sat_s (binary v128)
            @simd I16x8AddSatU => visit_i16x8_add_sat_u (binary v128)
            @simd I16x8Sub => visit_i16x8_sub (binary v128)
            @simd I16x8SubSatS => visit_i16x8_sub_sat_s (binary v128)
            @simd I16x8SubSatU => visit_i16x8_sub_sat_u (binary v128)
            @simd I16x8Mul => visit_i16x8_mul (binary v128)
            @simd I16x8MinS => visit_i16x8_min_s (binary v128)
            @simd I16x8MinU => visit_i16x8_min_u (binary v128)
            @simd I16x8MaxS => visit_i16x8_max_s (binary v128)
            @simd I16x8MaxU => visit_i16x8_max_u (binary v128)
            @simd I16x8AvgrU => visit_i16x8_avgr_u (binary v128)
            @simd I16x8ExtMulLowI8x16S => visit_i16x8_extmul_low_i8x16_s (binary v128)
            @simd I16x8ExtMulHighI8x16S => visit_i16x8_extmul_high_i8x16_s (binary v128)
            @simd I16x8ExtMulLowI8x16U => visit_i16x8_extmul_low_i8x16_u (binary v128)
            @simd I16x8ExtMulHighI8x16U => visit_i16x8_extmul_high_i8x16_u (binary v128)
            @simd I32x4ExtAddPairwiseI16x8S => visit_i32x4_extadd_pairwise_i16x8_s (unary v128)
            @simd I32x4ExtAddPairwiseI16x8U => visit_i32x4_extadd_pairwise_i16x8_u (unary v128)
            @simd I32x4Abs => visit_i32x4_abs (unary v128)
            @simd I32x4Neg => visit_i32x4_neg (unary v128)
            @simd I32x4AllTrue => visit_i32x4_all_true (test v128)
            @simd I32x4Bitmask => visit_i32x4_bitmask (test v128)
            @simd I32x4ExtendLowI16x8S => visit_i32x4_extend_low_i16x8_s (unary v128)
            @simd I32x4ExtendHighI16x8S => visit_i32x4_extend_high_i16x8_s (unary v128)
            @simd I32x4ExtendLowI16x8U => visit_i32x4_extend_low_i16x8_u (unary v128)
            @simd I32x4ExtendHighI16x8U => visit_i32x4_extend_high_i16x8_u (unary v128)
            @simd I32x4Shl => visit_i32x4_shl (shift v128)
            @simd I32x4ShrS => visit_i32x4_shr_s (shift v128)
            @simd I32x4ShrU => visit_i32x4_shr_u (shift v128)
            @simd I32x4Add => visit_i32x4_add (binary v128)
            @simd I32x4Sub => visit_i32x4_sub (binary v128)
            @simd I32x4Mul => visit_i32x4_mul (binary v128)
            @simd I32x4MinS => visit_i32x4_min_s (binary v128)
            @simd I32x4MinU => visit_i32x4_min_u (binary v128)
            @simd I32x4MaxS => visit_i32x4_max_s (binary v128)
            @simd I32x4MaxU => visit_i32x4_max_u (binary v128)
            @simd I32x4DotI16x8S => visit_i32x4_dot_i16x8_s (binary v128)
            @simd I32x4ExtMulLowI16x8S => visit_i32x4_extmul_low_i16x8_s (binary v128)
            @simd I32x4ExtMulHighI16x8S => visit_i32x4_extmul_high_i16x8_s (binary v128)
            @simd I32x4ExtMulLowI16x8U => visit_i32x4_extmul_low_i16x8_u (binary v128)
            @simd I32x4ExtMulHighI16x8U => visit_i32x4_extmul_high_i16x8_u (binary v128)
            @simd I64x2Abs => visit_i64x2_abs (unary v128)
            @simd I64x2Neg => visit_i64x2_neg (unary v128)
            @simd I64x2AllTrue => visit_i64x2_all_true (test v128)
            @simd I64x2Bitmask => visit_i64x2_bitmask (test v128)
            @simd I64x2ExtendLowI32x4S => visit_i64x2_extend_low_i32x4_s (unary v128)
            @simd I64x2ExtendHighI32x4S => visit_i64x2_extend_high_i32x4_s (unary v128)
            @simd I64x2ExtendLowI32x4U => visit_i64x2_extend_low_i32x4_u (unary v128)
            @simd I64x2ExtendHighI32x4U => visit_i64x2_extend_high_i32x4_u (unary v128)
            @simd I64x2Shl => visit_i64x2_shl (shift v128)
            @simd I64x2ShrS => visit_i64x2_shr_s (shift v128)
            @simd I64x2ShrU => visit_i64x2_shr_u (shift v128)
            @simd I64x2Add => visit_i64x2_add (binary v128)
            @simd I64x2Sub => visit_i64x2_sub (binary v128)
            @simd I64x2Mul => visit_i64x2_mul (binary v128)
            @simd I64x2ExtMulLowI32x4S => visit_i64x2_extmul_low_i32x4_s (binary v128)
            @simd I64x2ExtMulHighI32x4S => visit_i64x2_extmul_high_i32x4_s (binary v128)
            @simd I64x2ExtMulLowI32x4U => visit_i64x2_extmul_low_i32x4_u (binary v128)
            @simd I64x2ExtMulHighI32x4U => visit_i64x2_extmul_high_i32x4_u (binary v128)
            @simd F32x4Ceil => visit_f32x4_ceil (unary v128f)
            @simd F32x4Floor => visit_f32x4_floor (unary v128f)
            @simd F32x4Trunc => visit_f32x4_trunc (unary v128f)
            @simd F32x4Nearest => visit_f32x4_nearest (unary v128f)
            @simd F32x4Abs => visit_f32x4_abs (unary v128f)
            @simd F32x4Neg => visit_f32x4_neg (unary v128f)
            @simd F32x4Sqrt => visit_f32x4_sqrt (unary v128f)
            @simd F32x4Add => visit_f32x4_add (binary v128f)
            @simd F32x4Sub => visit_f32x4_sub (binary v128f)
            @simd F32x4Mul => visit_f32x4_mul (binary v128f)
            @simd F32x4Div => visit_f32x4_div (binary v128f)
            @simd F32x4Min => visit_f32x4_min (binary v128f)
            @simd F32x4Max => visit_f32x4_max (binary v128f)
            @simd F32x4PMin => visit_f32x4_pmin (binary v128f)
            @simd F32x4PMax => visit_f32x4_pmax (binary v128f)
            @simd F64x2Ceil => visit_f64x2_ceil (unary v128f)
            @simd F64x2Floor => visit_f64x2_floor (unary v128f)
            @simd F64x2Trunc => visit_f64x2_trunc (unary v128f)
            @simd F64x2Nearest => visit_f64x2_nearest (unary v128f)
            @simd F64x2Abs => visit_f64x2_abs (unary v128f)
            @simd F64x2Neg => visit_f64x2_neg (unary v128f)
            @simd F64x2Sqrt => visit_f64x2_sqrt (unary v128f)
            @simd F64x2Add => visit_f64x2_add (binary v128f)
            @simd F64x2Sub => visit_f64x2_sub (binary v128f)
            @simd F64x2Mul => visit_f64x2_mul (binary v128f)
            @simd F64x2Div => visit_f64x2_div (binary v128f)
            @simd F64x2Min => visit_f64x2_min (binary v128f)
            @simd F64x2Max => visit_f64x2_max (binary v128f)
            @simd F64x2PMin => visit_f64x2_pmin (binary v128f)
            @simd F64x2PMax => visit_f64x2_pmax (binary v128f)
            @simd I32x4TruncSatF32x4S => visit_i32x4_trunc_sat_f32x4_s (unary v128f)
            @simd I32x4TruncSatF32x4U => visit_i32x4_trunc_sat_f32x4_u (unary v128f)
            @simd F32x4ConvertI32x4S => visit_f32x4_convert_i32x4_s (unary v128f)
            @simd F32x4ConvertI32x4U => visit_f32x4_convert_i32x4_u (unary v128f)
            @simd I32x4TruncSatF64x2SZero => visit_i32x4_trunc_sat_f64x2_s_zero (unary v128f)
            @simd I32x4TruncSatF64x2UZero => visit_i32x4_trunc_sat_f64x2_u_zero (unary v128f)
            @simd F64x2ConvertLowI32x4S => visit_f64x2_convert_low_i32x4_s (unary v128f)
            @simd F64x2ConvertLowI32x4U => visit_f64x2_convert_low_i32x4_u (unary v128f)
            @simd F32x4DemoteF64x2Zero => visit_f32x4_demote_f64x2_zero (unary v128f)
            @simd F64x2PromoteLowF32x4 => visit_f64x2_promote_low_f32x4 (unary v128f)

            // Relaxed SIMD operators
            // https://github.com/WebAssembly/relaxed-simd
            @relaxed_simd I8x16RelaxedSwizzle => visit_i8x16_relaxed_swizzle (binary v128)
            @relaxed_simd I32x4RelaxedTruncF32x4S => visit_i32x4_relaxed_trunc_f32x4_s (unary v128)
            @relaxed_simd I32x4RelaxedTruncF32x4U => visit_i32x4_relaxed_trunc_f32x4_u (unary v128)
            @relaxed_simd I32x4RelaxedTruncF64x2SZero => visit_i32x4_relaxed_trunc_f64x2_s_zero (unary v128)
            @relaxed_simd I32x4RelaxedTruncF64x2UZero => visit_i32x4_relaxed_trunc_f64x2_u_zero (unary v128)
            @relaxed_simd F32x4RelaxedMadd => visit_f32x4_relaxed_madd (ternary v128)
            @relaxed_simd F32x4RelaxedNmadd => visit_f32x4_relaxed_nmadd (ternary v128)
            @relaxed_simd F64x2RelaxedMadd => visit_f64x2_relaxed_madd  (ternary v128)
            @relaxed_simd F64x2RelaxedNmadd => visit_f64x2_relaxed_nmadd (ternary v128)
            @relaxed_simd I8x16RelaxedLaneselect => visit_i8x16_relaxed_laneselect (ternary v128)
            @relaxed_simd I16x8RelaxedLaneselect => visit_i16x8_relaxed_laneselect (ternary v128)
            @relaxed_simd I32x4RelaxedLaneselect => visit_i32x4_relaxed_laneselect (ternary v128)
            @relaxed_simd I64x2RelaxedLaneselect => visit_i64x2_relaxed_laneselect (ternary v128)
            @relaxed_simd F32x4RelaxedMin => visit_f32x4_relaxed_min (binary v128)
            @relaxed_simd F32x4RelaxedMax => visit_f32x4_relaxed_max (binary v128)
            @relaxed_simd F64x2RelaxedMin => visit_f64x2_relaxed_min (binary v128)
            @relaxed_simd F64x2RelaxedMax => visit_f64x2_relaxed_max (binary v128)
            @relaxed_simd I16x8RelaxedQ15mulrS => visit_i16x8_relaxed_q15mulr_s (binary v128)
            @relaxed_simd I16x8RelaxedDotI8x16I7x16S => visit_i16x8_relaxed_dot_i8x16_i7x16_s (binary v128)
            @relaxed_simd I32x4RelaxedDotI8x16I7x16AddS => visit_i32x4_relaxed_dot_i8x16_i7x16_add_s (ternary v128)
        }
    };
}
