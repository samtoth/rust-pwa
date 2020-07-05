import argparse
import subprocess

def build_wasm():
    subprocess.run("wasm-pack build --target web --out-name wasm --out-dir static", shell=True, check=True, cwd="./frontend")
    

def run(release: bool):
    build_wasm()
    if release:
        subprocess.run("cargo run --release", shell=True)
    else:
        subprocess.run("cargo run", shell=True)

def main():
    parser = argparse.ArgumentParser(description='Build tools for my application.')
    
    parser.add_argument('-R', '--release', action='store_true', help='Build artifacts in release mode, with optimizations', required=False)

    args = parser.parse_args()
    release = args.release

    run(release)



if __name__ == '__main__':
    main()