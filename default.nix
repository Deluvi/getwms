with (import <nixpkgs> {});

rustPlatform.buildRustPackage rec {
  pname = "getwms";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "1z4cs8ayf7p115kh5sdqsjfdi1v5kgshkp4zrzs2pzz1gm42zaza";

  buildInputs = [ pkgconfig openssl ];

  meta = with lib; {
    description = "A CLI to collect webmensions from common public webmentions backends";
    homepage = "https://github.com/deluvi/getwms";
    maintainers = with maintainers; [deluvi];
    platforms = platforms.all;
  };
}
