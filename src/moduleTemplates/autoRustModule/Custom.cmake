# Set the path to the pre-built Rust library:
set(RUST_LIB_PATH "${CMAKE_SOURCE_DIR}/moduleTemplates/autoRustModule/target/release/libautoRustModule.so")

# Adjust include directories
target_include_directories(${TARGET_NAME} PRIVATE ${CMAKE_SOURCE_DIR}/moduleTemplates/autoRustModule)

# Tell the linker to use the Rust library
target_link_libraries(${TARGET_NAME} PRIVATE "${RUST_LIB_PATH}")

# Remove compilation of c files:
# this instruction makes sure that the compiler doesn't compile `autoRustModule.c` since it does not exists
set_source_files_properties(${CMAKE_SOURCE_DIR}/moduleTemplates/autoRustModule/autoRustModule.c PROPERTIES GENERATED TRUE)

# Force include of the library for debugging purposes
set_property(TARGET ${TARGET_NAME} PROPERTY LINK_INTERFACE_LIBRARIES "${RUST_LIB_PATH}")
