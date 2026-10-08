const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{
        .default_target = .{
            .cpu_arch = .wasm32,
            .os_tag = .freestanding,
            .cpu_features_add = std.Target.wasm.featureSet(&.{ .simd128, .bulk_memory }),
        },
    });
    const optimize = b.standardOptimizeOption(.{
        .preferred_optimize_mode = .ReleaseSmall,
    });

    const lib_mod = b.createModule(.{
        .target = target,
        .optimize = optimize,
    });
    lib_mod.addIncludePath(b.path("../c/lib25519/wasm/include"));
    lib_mod.addIncludePath(b.path("../c/lib25519/cryptoint"));
    lib_mod.addIncludePath(b.path("../c/lib25519/crypto_sign/ed25519/ref10"));
    lib_mod.addIncludePath(b.path("../c/lib25519/crypto_hashblocks/sha512/m3"));

    const base_flags = [_][]const u8{
        "-O2",
        "-flto",
        "-std=c99",
        "-fwrapv",
        "-fvisibility=hidden",
        "-fno-stack-protector",
        "-Wall",
        "-Wextra",
        "-Wno-unused-parameter",
    };
    const ng_flags = base_flags ++ [_][]const u8{ "-include", "../c/lib25519/wasm/include/ns_ng.h" };
    const mgnp_flags = base_flags ++ [_][]const u8{ "-include", "../c/lib25519/wasm/include/ns_mgnp.h" };
    const pow_flags = base_flags ++ [_][]const u8{ "-include", "../c/lib25519/wasm/include/ns_pow.h" };
    const sign_flags = base_flags ++ [_][]const u8{ "-include", "../c/lib25519/wasm/include/ns_sign.h" };
    const common_flags = base_flags ++ [_][]const u8{ "-include", "../c/lib25519/wasm/include/ns_common.h" };

    lib_mod.addCSourceFiles(.{
        .files = &.{
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_0.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_1.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_add.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_cmov.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_copy.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_frombytes.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_invert.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_isnegative.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_mul.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_neg.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_sq.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_sq2.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_sub.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/fe_tobytes.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_madd.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p1p1_to_p2.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p1p1_to_p3.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p2_dbl.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p3_0.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p3_dbl.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_p3_to_p2.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_precomp_0.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/ge_scalarmult_base.c",
            "../c/lib25519/crypto_nG/merged25519/ref10/shared-base.c",
            "../c/lib25519/wasm/nG_ed25519.c",
        },
        .flags = &ng_flags,
    });

    lib_mod.addCSourceFiles(.{
        .files = &.{
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_0.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_1.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_add.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_copy.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_frombytes.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_invert.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_isnegative.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_isnonzero.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_mul.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_neg.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_pow22523.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_sq.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_sq2.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_sub.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/fe_tobytes.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_add.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_double_scalarmult.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_frombytes.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_madd.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_msub.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p1p1_to_p2.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p1p1_to_p3.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p2_0.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p2_dbl.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p3_dbl.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p3_to_cached.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_p3_to_p2.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_sub.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/ge_tobytes.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/mGnP.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/sc_reduce.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/shared-Bi.c",
        },
        .flags = &mgnp_flags,
    });

    lib_mod.addCSourceFiles(.{
        .files = &.{
            "../c/lib25519/crypto_pow/inv25519/ref10/api.c",
            "../c/lib25519/crypto_pow/inv25519/ref10/fe_frombytes.c",
            "../c/lib25519/crypto_pow/inv25519/ref10/fe_invert.c",
            "../c/lib25519/crypto_pow/inv25519/ref10/fe_mul.c",
            "../c/lib25519/crypto_pow/inv25519/ref10/fe_sq.c",
            "../c/lib25519/crypto_pow/inv25519/ref10/fe_tobytes.c",
        },
        .flags = &pow_flags,
    });

    lib_mod.addCSourceFiles(.{
        .files = &.{
            "../c/lib25519/crypto_sign/ed25519/ref10/sign.c",
            "../c/lib25519/crypto_sign/ed25519/ref10/sc_muladd.c",
            "../c/lib25519/crypto_mGnP/ed25519/ref10/sc_reduce.c",
        },
        .flags = &sign_flags,
    });

    lib_mod.addCSourceFiles(.{
        .files = &.{
            "../c/lib25519/crypto_hashblocks/sha512/m3/constants.c",
            "../c/lib25519/crypto_hashblocks/sha512/wflip/inner.c",
            "../c/lib25519/crypto_hash/sha512/ref/hash.c",
            "../c/lib25519/crypto_verify/32/ref/verify.c",
            "../c/lib25519/crypto_sign/ed25519/amd64/keypair.c",
            "../c/lib25519/crypto_sign/ed25519/amd64/open.c",
            "../c/lib25519/cryptoint/int8_optblocker.c",
            "../c/lib25519/cryptoint/int16_optblocker.c",
            "../c/lib25519/cryptoint/int32_optblocker.c",
            "../c/lib25519/cryptoint/int64_optblocker.c",
            "../c/lib25519/cryptoint/uint8_optblocker.c",
            "../c/lib25519/cryptoint/uint16_optblocker.c",
            "../c/lib25519/cryptoint/uint32_optblocker.c",
            "../c/lib25519/cryptoint/uint64_optblocker.c",
            "../c/lib25519/wasm/randombytes.c",
        },
        .flags = &common_flags,
    });

    const lib = b.addLibrary(.{
        .name = "ed25519",
        .root_module = lib_mod,
        .linkage = .static,
    });
    b.installArtifact(lib);
}
