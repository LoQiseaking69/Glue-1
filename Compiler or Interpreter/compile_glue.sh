compile_script = f"""
# Compile Script for Glue Language

# Compile to LLVM IR
clang -S -emit-llvm glue_program.py -o {output_file}.ll

# Optimize LLVM IR
opt {compiler_flags} {output_file}.ll -o {output_file}_opt.ll

# Generate Object File
clang -c {output_file}_opt.ll -o {output_file}.o

# Link Object File
clang {linker_flags} {library_dependencies} {output_file}.o -o {output_file}
"""

print(compile_script).
