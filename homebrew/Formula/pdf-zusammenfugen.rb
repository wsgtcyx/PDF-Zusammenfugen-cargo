class PdfZusammenfugen < Formula
  desc "Merge multiple PDF files into one from the command line"
  homepage "https://pdfzus.de/"
  url "https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "REPLACE_WITH_V0_2_0_TARBALL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    assert_match "Usage", shell_output("#{bin}/pdf-zusammenfugen --help")
    output = shell_output("#{bin}/pdf-zusammenfugen -o out.pdf 2>&1", 2)
    assert_match "Bitte mindestens eine Eingabe-PDF", output
  end
end
