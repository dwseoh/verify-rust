c_rust_llvm(custom-vec-loop-unroll custom-vec-loop-unroll.c)

configure_file(sea.yaml sea.yaml @ONLY)
sea_add_unsat_test(custom-vec-loop-unroll)
