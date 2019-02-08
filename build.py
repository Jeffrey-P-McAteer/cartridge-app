#!/usr/bin/env python3

# Script responsible for performing high-level builds,
# including using cross to cross compile native windows binaries.

import sys
if sys.version_info[0] < 3:
  raise Exception("Script requires Python 3")

import os, subprocess, socket

def assume_exists(whatis: str, path: str):
  if os.path.exists(path):
    print("{}: {}".format(whatis, path))
  else:
    print("ERROR: {} not found at {}".format(whatis, path))
    sys.exit(5)

APP_NAME = "cartridge_app"

if "linux" in sys.platform:
  WIN_CROSS_TARGETS = [
    #"x86_64-pc-windows-gnu", # Cross-compiling with the SDL2 dependency is non-trivial
  ]
  NATIVE_CARGO_TARGETS = [
    "x86_64-unknown-linux-gnu",
  ]
elif "win" in sys.platform:
  WIN_CROSS_TARGETS = [ ]
  NATIVE_CARGO_TARGETS = [
    "x86_64-pc-windows-gnu",
  ]
else:
  print("Error unsupported build OS")
  sys.exit(2)

if __name__ == '__main__':
  if socket.gethostname() == "azure-angel" and "linux" in sys.platform:
    # Jeffrey's laptop, let's boost CPU because I can't really use an alias here
    print("Detected Jeffrey's laptop, boosting CPU before compile...")
    subprocess.check_output(["set-cpu", "game"])
    
  #print("Building native library and native binary...")
  #subprocess.call(["cargo", "build", "--release"])
  #assume_exists("Native binary", "./target/release/"+APP_NAME+"")
  
  for tgt in WIN_CROSS_TARGETS:
    print("Compiling shared library and binary for {}".format(tgt))
    subprocess.call(["cross", "build", "--release", "--target", tgt])
    assume_exists("Native binary", "./target/{}/release/{}.exe".format(tgt, APP_NAME))
    
  for tgt in NATIVE_CARGO_TARGETS:
    print("Compiling shared library and binary for {}".format(tgt))
    subprocess.call(["cargo", "build", "--release", "--target", tgt])
    if "linux" in sys.platform:
      assume_exists("Native binary", "./target/{}/release/{}".format(tgt, APP_NAME))
    else:
      assume_exists("Native binary", "./target/{}/release/{}.exe".format(tgt, APP_NAME))
    
  if socket.gethostname() == "azure-angel":
    print("Detected Jeffrey's laptop, pushing built artifacts to cs.odu.edu/~jmcateer/"+APP_NAME+"/")
    # check directory
    subprocess.call(["ssh", "cs", "if ! [ -d ./secure_html/"+APP_NAME+"/ ] ; then mkdir ./secure_html/"+APP_NAME+"/ ; fi"])
    # copy dl_page
    subprocess.call(["scp", "./dl_page.html", "cs:./secure_html/"+APP_NAME+"/index.html"])
    # Copy built targets
    for tgt in WIN_CROSS_TARGETS:
      subprocess.call(["scp",
        "./target/{}/release/{}.exe".format(tgt, APP_NAME),
        "cs:./secure_html/"+APP_NAME+"/"+APP_NAME+"-{}.exe".format(tgt)
      ])
    for tgt in NATIVE_CARGO_TARGETS:
      if "linux" in sys.platform:
        subprocess.call(["scp",
          "./target/{}/release/{}".format(tgt, APP_NAME),
          "cs:./secure_html/"+APP_NAME+"/"+APP_NAME+"-{}".format(tgt)
        ])
      else:
        subprocess.call(["scp",
          "./target/{}/release/{}.exe".format(tgt, APP_NAME),
          "cs:./secure_html/"+APP_NAME+"/"+APP_NAME+"-{}.exe".format(tgt)
        ])
    subprocess.call(["ssh", "cs", "chmod -R a+rx ./secure_html/"+APP_NAME+"/"])
    # Print web directory
    subprocess.call(["ssh", "cs", "ls -alh ./secure_html/"+APP_NAME+"/"])
    