class Cargonode < Formula
    version '0.1.0'
    desc "Streamline Node.js development workflows"
    homepage "https://github.com/xosnrdev/cargonode"
  
    if OS.mac?
        url "https://github.com/xosnrdev/cargonode/releases/download/#{version}/cargonode-#{version}-x86_64-apple-darwin.tar.gz"
        sha256 "879d120c3b15cd1cd87fa0b3f4dc4fa54ad6c632712d9a8646be7697863fbb45"
    elsif OS.linux?
        url "https://github.com/xosnrdev/cargonode/releases/download/#{version}/cargonode-#{version}-x86_64-unknown-linux-musl.tar.gz"
        sha256 "eda2b34b131071a7ae5e8e30df13b7883a63d06038a26ecbb729c376d7df34fe"
    end
  
    def install
      bin.install "cargonode"
    end

end