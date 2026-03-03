class PdfZusammenfugenCargo < Formula
  desc "Rust-Tool zum pdf zusammenfuegen, pdf verbinden und pdf mergen"
  homepage "https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo"
  url "https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_RELEASE_TARBALL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "pdf-zusammenfugen-cargo", shell_output("#{bin}/pdf-zusammenfugen-cargo --help")
  end
end
