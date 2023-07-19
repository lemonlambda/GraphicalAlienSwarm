{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {self, nixpkgs, flake-utils, fenix, naersk,  ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      target = "x86_64-unknown-linux-gnu";
      toolchain = with fenix.packages.${system}; combine [
        latest.cargo
        latest.rustc
        targets.${target}.latest.rust-std
      ];
      pkgs = import nixpkgs {
        overlays = [
          (_: super: let pkgs = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs pkgs)
        ];
        inherit system;
      };
      buildInputs = with pkgs; [
        rust-analyzer-nightly
        cargo-expand
        pkgsCross.mingwW64.buildPackages.gcc
        glibc_multi
        udev alsa-lib vulkan-loader
        xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
        libxkbcommon wayland
      ];
      src = ./.;
      copySources = [
        "graphical_alien_swarm_proc_macros"
      ];
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in {
      packages.default = self.packages.${system}.packageGame;

      packages.packageGame = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage rec {
        singleStep = true;
        pname = manifest.name;
        version = manifest.version;
        gameName = "${pname}-${version}";

        inherit src copySources buildInputs;
        
        nativeBuildInputs = with pkgs; [
          toolchain
          pkg-config
        ];


        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        CARGO_BUILD_TARGET = target;
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
          let
            inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
          in
          "${cc}/bin/${cc.targetPrefix}cc";
      };

      devShells.${system}.default = pkgs.mkShell {
        inherit src copySources buildInputs;
        nativeBuildInputs = with pkgs; [ toolchain ];

        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        CARGO_BUILD_TARGET = target;
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
          let
            inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
          in
          "${cc}/bin/${cc.targetPrefix}cc";
      };
  });
}