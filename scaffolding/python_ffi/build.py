import os
import subprocess
import glob
import time

CWD = os.path.dirname(os.path.realpath(__file__)) 

def check_cargo_install():
    try:
        subprocess.run(("cargo", "--version"), check=True, capture_output=True)
    except:
        print("ERROR: Unable to locate the cargo binary.\nIs Rust installed on this system (https://www.rust-lang.org/tools/install)?\nTry closing and opening a new terminal if so.")
        exit()

def install_maturin():
    subprocess.run(("python3", "-m", "pip", "install", "maturin[patchelf]"), check=False)
    subprocess.run(("python3", "-m", "pip", "install", "maturin"), check=True)

def install_windows():
    for prev_wheel in glob.glob(r".\target\wheels\ffi-*"):
        os.system(f"del {prev_wheel}")
    while glob.glob(r".\target\wheels\ffi-*"):
        time.sleep(.5)

    os.system("python3 -m maturin build --release --interpreter python3")
    while not glob.glob(r".\target\wheels\ffi-*"):
        time.sleep(.5)

    wheel = glob.glob(r".\target\wheels\ffi-*")[0]
    os.system(f"python3 -m pip install --force-reinstall {wheel}")

def install_posix(target_loc):
    subprocess.run(("python3",  "-m" , "maturin","build","--release"), check=True)
    dylib = "./target/release/libffi.dylib"
    so = "./target/release/libffi.so"

    if os.path.isfile(dylib):
        subprocess.run(("cp", dylib, target_loc), check=True)
    elif os.path.isfile(so):
        subprocess.run(("cp", so, target_loc), check=True)
    else:
        print("ERROR: COULD NOT FIND THE LIBRARY FILE")
        exit(1)

os_name = os.name.lower() 

if __name__ == "__main__":
    LAUNCHED_FROM = os.getcwd()
    print("LAUNCHED_FROM", LAUNCHED_FROM)

    os.chdir(CWD)
    check_cargo_install()
    install_maturin()
    if os_name == "windows" or os_name == "nt":
        install_windows()
    elif os_name == "posix":
        install_posix(os.path.join(LAUNCHED_FROM, "ffi.so"))
    else:
        print(f"ERROR: UNSURE HOW TO INSTALL ON THIS SYSTEM! {os_name}")
        exit()
    print(">>> Successfully built Python-Rust library! Happy coding :D <<<")
