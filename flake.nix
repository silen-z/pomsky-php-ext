{
  description = "PHP extension for Pomsky regex language";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = (import nixpkgs) {
            inherit system;
          };
          
          naersk' = pkgs.callPackage naersk {};
          
          pomskyPhpExt = naersk'.buildPackage {
            pname = "php_pomsky";
            version = "0.1.0";

            src = ./pomsky_php_ext;
  
            nativeBuildInputs = [
              pkgs.php81.unwrapped
              pkgs.rustPlatform.bindgenHook
            ];
            
            postInstall = ''
              mkdir -p $out/lib/php/extensions
              cp $out/lib/libphp_pomsky.so $out/lib/php/extensions/pomsky.so
            '';
            
            copyBins = false;
            copyLibs = true;

            PHP = "${pkgs.php81}/bin/php";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };
          
          phpWithPomsky = pkgs.php81.withExtensions ({ ... }: [ pomskyPhpExt ]);

          phpWithFfi = pkgs.php81.withExtensions ({ all, ... }: [ all.ffi ]);

        in
        {
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ phpWithFfi pkgs.cargo ];
            };

            phpWithPomsky = pkgs.mkShell {
              buildInputs = [ phpWithPomsky ];
            };
          };
        }
      );
}