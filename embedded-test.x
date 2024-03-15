SECTIONS
{
  .embedded_test 1 (INFO) :
  {
    KEEP(*(.embedded_test.*));
    PROVIDE(embedded_test_linker_file_not_added_to_rustflags = .);
  }
}
