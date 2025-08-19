# This linker script is needed to ensure our version + testcase symbols are not optimized away
# The EMBEDDED_TEST_VERSION symbol is needed by probe-rs to determine whether a binary contains embedded tests or not
# Afterwards it reads the testcases from the .embedded_test section

# Redirect/rename a function here, so that we can make sure the user has added the linker script to the RUSTFLAGS
EXTERN (__embedded_test_start);
PROVIDE(embedded_test_linker_file_not_added_to_rustflags = __embedded_test_start);

PROVIDE(_embedded_test_setup = __embedded_test_default_setup);

# Define a section for the embedded tests and make sure it is not optimized away
SECTIONS
{
  .embedded_test 1 (INFO) :
  {
    KEEP(*(.embedded_test.*));
  }
}

# NOTE: build.rs will add a `INSERT AFTER .comment;` here, if we're compiling for std