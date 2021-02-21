###############################################
{ pkgs ? import <nixpkgs> {} }:

with pkgs;
stdenv.mkDerivation {
  name = "datalove";

  buildInputs = with pkgs; [
    cargo
    git
  ];

  # src = fetchurl {
  #   url = "mirror://gnu/hello/${name}.tar.gz";
  #   sha256 = "0ssi1wpaf7plaswqqjwigppsg5fyh99vdlb9kzl7c9lng89ndq1i";
  # };

  doCheck = true;

  meta = {
    description = "";
    longDescription = ''
    '';
    homepage = https://github.com/datalove-app/datalove;
    license = stdenv.lib.licenses.mit;
    maintainers = [  ];
    # platforms = platforms.all;
  };
}
