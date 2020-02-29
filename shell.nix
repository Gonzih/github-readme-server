let
   pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "vuln-scanner-dev";
  buildInputs = with pkgs; [
    stdenv
    pkgconfig
    rustup
    cargo
    openssl
    llvm
    llvmPackages.libclang
  ];
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
  '';
}
