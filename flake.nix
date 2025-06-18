{
  description = "Rust + Jupyter development environment with russell dependencies";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Create a Python environment with all Jupyter components
        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
          jupyter
          jupyterlab
          notebook
          ipykernel
          nbconvert
          ipywidgets
        ]);
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            cargo
            rustc

            # Use the complete Python environment
            pythonEnv

            # For evcxr (Rust Jupyter kernel)
            cmake
            pkg-config
            openssl

            # Russell dependencies
            lapack
            openblas
            suitesparse

            # Additional build tools that might be needed
            gfortran

            # Nice to have
            rustfmt
            clippy
          ];

          # Environment variables that russell might need
          PKG_CONFIG_PATH = "${pkgs.lapack}/lib/pkgconfig:${pkgs.openblas}/lib/pkgconfig:${pkgs.suitesparse}/lib/pkgconfig";

          shellHook = ''
            echo "🦀 Rust + Jupyter environment loaded!"
            echo "Rust version: $(rustc --version)"
            echo "Python version: $(python --version)"
            echo "LAPACK, OpenBLAS, and SuiteSparse are available"

            # Install evcxr_jupyter if not already installed
            if ! command -v evcxr_jupyter &> /dev/null; then
              echo "Installing evcxr_jupyter (Rust kernel for Jupyter)..."
              cargo install evcxr_jupyter
              evcxr_jupyter --install
            fi

            echo ""
            echo "To start Jupyter, you can use:"
            echo "  jupyter lab      # for JupyterLab interface"
            echo "  jupyter notebook # for classic notebook interface"
          '';
        };
      });
}
