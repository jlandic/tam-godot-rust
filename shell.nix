# MIT License

# Copyright (c) 2020 The godot-rust developers

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

with import <nixpkgs> {
  config.android_sdk.accept_license = true;
};

stdenv.mkDerivation {
  name = "tamrl";
  nativeBuildInputs = [
    cacert
    emacs
    godot
    jdk
    nix
    openssl
    pkgconfig
    sccache
  ];
  buildInputs = [
    alsaLib
    clang
    glibc_multi
    libGL
    libpulseaudio
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXinerama
    xorg.libXrandr
    xorg.libXrender
    zlib
  ];
  shellHook = ''
    export PATH=$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH
  '';

  # Set Environment Variables
  EDITOR = "emacs";
  LD_LIBRARY_PATH = builtins.concatStringsSep ":"  [
    "${alsaLib}/lib/"
    "${libGL}/lib/"
    "${libpulseaudio}/lib/"
    "${xorg.libX11}/lib/"
    "${xorg.libXcursor}/lib/"
    "${xorg.libXi}/lib/"
    "${xorg.libXinerama}/lib/"
    "${xorg.libXrandr}/lib/"
    "${xorg.libXrender}/lib/"
    "${zlib}/lib/"
  ];
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  RUST_BACKTRACE = 1;
  RUSTC_WRAPPER = "${sccache}/bin/sccache";
}
