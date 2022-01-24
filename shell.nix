with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "tonic-axum";
  buildInputs = [
    protobuf
  ];
  PROTOC = "${pkgs.protobuf}/bin/protoc";
}
