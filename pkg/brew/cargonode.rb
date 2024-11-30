class Cargonode < Formula
    desc "Streamline Node.js development workflows"
    homepage "https://github.com/xosnrdev/cargonode"
    url "https://github.com/xosnrdev/cargonode/archive/refs/tags/0.1.0.tar.gz"
    sha256 "42a936c2e863ebca38319c976dd229c38d0b5040862b14c6cf450b54ec7e5276"
    license "Apache-2.0"
    head "https://github.com/xosnrdev/cargonode.git", branch: "master"
  
    livecheck do
      url :stable
      strategy :github_latest
    end
  
    bottle do
        sha256 cellar: :any,                 arm64_sonoma:   "f2e7750b013fc8a01d759edc1a8e200721471d241fafcf045d71c36b9129cad2"
        sha256 cellar: :any,                 arm64_ventura:  "f2e7750b013fc8a01d759edc1a8e200721471d241fafcf045d71c36b9129cad2"
        sha256 cellar: :any,                 arm64_monterey: "f2e7750b013fc8a01d759edc1a8e200721471d241fafcf045d71c36b9129cad2"
        sha256 cellar: :any,                 sonoma:         "879d120c3b15cd1cd87fa0b3f4dc4fa54ad6c632712d9a8646be7697863fbb45"
        sha256 cellar: :any,                 ventura:        "879d120c3b15cd1cd87fa0b3f4dc4fa54ad6c632712d9a8646be7697863fbb45"
        sha256 cellar: :any,                 monterey:       "879d120c3b15cd1cd87fa0b3f4dc4fa54ad6c632712d9a8646be7697863fbb45"
        sha256 cellar: :any_skip_relocation, x86_64_linux:   "eda2b34b131071a7ae5e8e30df13b7883a63d06038a26ecbb729c376d7df34fe"
    end
  
    def install
      bin.install "cargonode"
    end
  
    test do
        system "#{bin}/cargonode", "--version"
    end
end