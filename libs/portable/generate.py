import os
import optparse
import brotli

# file compress level(0-11)
compress_level = 11
# 4GB maximum
length_count = 4
# encoding
encoding = 'utf-8'




def build_portable(output_folder: str):
    os.chdir(output_folder)
    os.system("cargo build --release")

# Linux: python3 generate.py -f ../rustdesk-portable-packer/test -o . -e ./test/main.py
# Windows: python3 .\generate.py -f ..\rustdesk\flutter\build\windows\runner\Debug\ -o . -e ..\rustdesk\flutter\build\windows\runner\Debug\rustdesk.exe


if __name__ == '__main__':
    parser = optparse.OptionParser()
    parser.add_option("-f", "--folder", dest="folder",
                      help="folder to compress")
    parser.add_option("-o", "--output", dest="output_folder",
                      help="the root of portable packer project")
    parser.add_option("-e", "--executable", dest="executable",
                      help="specify startup file")
    (options, args) = parser.parse_args()
    folder = options.folder
    output_folder = os.path.abspath(options.output_folder)

    exe: str = os.path.abspath(options.executable)
    if not exe.startswith(os.path.abspath(folder)):
        print("the executable must locate in source folder")
        exit(-1)
    exe = '.' + exe[len(os.path.abspath(folder)):]
    print("executable path: " + exe)
    build_portable(output_folder)
