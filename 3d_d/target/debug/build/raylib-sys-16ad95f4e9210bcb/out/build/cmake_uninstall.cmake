if(NOT EXISTS "/Users/juanluis/Documents/educacion/U/semestres/semestre_10/Graficas/3d_d/target/debug/build/raylib-sys-16ad95f4e9210bcb/out/build/install_manifest.txt")
  message(FATAL_ERROR "Cannot find install manifest: /Users/juanluis/Documents/educacion/U/semestres/semestre_10/Graficas/3d_d/target/debug/build/raylib-sys-16ad95f4e9210bcb/out/build/install_manifest.txt")
endif()

file(READ "/Users/juanluis/Documents/educacion/U/semestres/semestre_10/Graficas/3d_d/target/debug/build/raylib-sys-16ad95f4e9210bcb/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach(file ${files})
  message(STATUS "Uninstalling $ENV{DESTDIR}${file}")
  if(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    exec_program(
      "/opt/homebrew/bin/cmake" ARGS "-E remove \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
      )
    if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing $ENV{DESTDIR}${file}")
    endif()
  else(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    message(STATUS "File $ENV{DESTDIR}${file} does not exist.")
  endif()
endforeach()
