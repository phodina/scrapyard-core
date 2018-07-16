if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --features cc)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --features cc --release)
    set(TARGET_DIR "release")
endif ()

set(MCU_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libmcu.so")

add_custom_target(mcu ALL
    COMMENT "Compiling mcu module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${MCU_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(mcu PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME mcu_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
