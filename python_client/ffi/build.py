import os
import subprocess

CWD = os.path.dirname(os.path.realpath(__file__)) 

def check_cargo_install():
    try:
        subprocess.run(("cargo", "--version"), check=True, capture_output=True)
    except:
        print("ERROR: Unable to locate the cargo binary.\nIs Rust installed on this system (https://www.rust-lang.org/tools/install)?\nTry closing and opening a new terminal if so.")
        exit()

def install_maturin():
    subprocess.run(("python3", "-m", "pip", "install", "maturin"), check=True)

def install_windows():
    subprocess.run(("maturin","build","--release"), check=True)
    dll = "./target/release/maturin/libffi.dll"
    target_loc = "../ffi.dll"

    if os.path.isfile(dll):
        subprocess.run(("copy", dll, target_loc), check=True)
    else:
        print("ERROR: COULD NOT FIND THE LIBRARY FILE ON WINDOWS")

def install_posix():
    subprocess.run(("maturin","build","--release"), check=True)
    dylib = "./target/release/maturin/libffi.dylib"
    so = "./target/release/maturin/libffi.so"
    target_loc = "../ffi.so"

    if os.path.isfile(dylib):
        subprocess.run(("cp", dylib, target_loc), check=True)
    elif os.path.isfile(so):
        subprocess.run(("cp", so, target_loc), check=True)
    else:
        print("ERROR: COULD NOT FIND THE LIBRARY FILE")

os_name = os.name.lower() 

if __name__ == "__main__":
    os.chdir(CWD)
    check_cargo_install()
    install_maturin()
    if os_name == "windows" or os_name == "nt":
        install_windows()
    elif os_name == "posix":
        install_posix()
    else:
        print(f"ERROR: UNSURE HOW TO INSTALL ON THIS SYSTEM! {os_name}")
        exit()
    print(">>> Successfully built! Happy coding :D <<<")
