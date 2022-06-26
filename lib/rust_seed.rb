module RustSeed
  extend FFI::Library

  ffi_lib "#{Rails.root}/ffi/diesel_ffi/target/release/libdiesel_ffi.so"
  attach_function :seed, [], :string

  seed
end
