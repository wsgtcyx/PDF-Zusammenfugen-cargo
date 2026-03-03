class PdfZusammenfugenCargo < Formula
  desc "Rust-Tool zum pdf zusammenfuegen, pdf verbinden und pdf mergen"
  homepage "https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo"
  url "https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "8d6115b1eb62ee484ad28ce7184f8e4184508c34ea31b4595857319ece6d3710"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "pdf-zusammenfugen-cargo", shell_output("#{bin}/pdf-zusammenfugen-cargo --help")
  end
end
