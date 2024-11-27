class cargonodeBin < Formula
    version '0.1.0'
    desc "Revolutionize Node.js development workflows."
    homepage "https://github.com/xosnrdev/cargonode"
  
    if OS.mac?
        url "https://github.com/xosnrdev/cargonode/releases/download/#{version}/cargonode-#{version}-x86_64-apple-darwin.tar.gz"
        sha256 ""
    elsif OS.linux?
        url "https://github.com/xosnrdev/cargonode/releases/download/#{version}/cargonode-#{version}-x86_64-unknown-linux-musl.tar.gz"
        sha256 ""
    end
  
    conflicts_with "cargonode"
  
    def install
      bin.install "cargonode"
      man1.install "doc/cargonode.1"
    end
  end