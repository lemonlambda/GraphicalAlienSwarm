{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {self, nixpkgs, ... }@inputs: let
    pkgs = nixpkgs.legacyPackages.${system}; 
  in {
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default = self.packages.${system}.packageGame;
    
      packages.packageGame = rustPlatform.buildRustPackage {
        name = "packageGame";
        version = "0.1.0";
        game_name = "AlienSwarm-${version}";
        target = system;
        src = self;
        buildInputs = with pkgs; [
          zip
        ];
        buildPhase = ''
          # Setup dirs
          mkdir -p $out/game
          mkdir -p $out/target
          mkdir -p $out/final

          # Build
          cargo build --target $target --release --target_dir $out/target --out-dir $out/final
        '';
        installPhase = ''
          cp $src/assets $out/game
          mv $out/final/* $out/game
          zip $game_name.zip $out/game -r
          mv $game_name.zip $src
        '';
      };
    }
  );
}