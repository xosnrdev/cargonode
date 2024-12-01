class Cargonode < Formula
    desc "Unified tooling for Node.js"
    homepage "https://github.com/xosnrdev/cargonode"
    url "https://github.com/xosnrdev/cargonode/archive/refs/tags/0.1.0.tar.gz"
    sha256 "8b9e092e9ff4327a23d1930fdcf7f565556c745d17a2c6c205e6c2b20722158c"
    license "Apache-2.0"
    head "https://github.com/xosnrdev/cargonode.git", branch: "master"
  
    livecheck do
      url :stable
      strategy :github_latest
    end

    depends_on "rust" => :build
  
    bottle do
      sha256 cellar: :any,                 arm64_sonoma:   "b83cb6528f53a2e044780ad5d00d791688bce1d0bf1feea9935199ec8a6d65fc"
      sha256 cellar: :any,                 arm64_ventura:  "b83cb6528f53a2e044780ad5d00d791688bce1d0bf1feea9935199ec8a6d65fc"
      sha256 cellar: :any,                 arm64_monterey: "b83cb6528f53a2e044780ad5d00d791688bce1d0bf1feea9935199ec8a6d65fc"
      sha256 cellar: :any,                 sonoma:         "0b4025f92ded6afbba98b4d14b8de806af3711e0f990ab155f897784512c66d4"
      sha256 cellar: :any,                 ventura:        "0b4025f92ded6afbba98b4d14b8de806af3711e0f990ab155f897784512c66d4"
      sha256 cellar: :any,                 monterey:       "0b4025f92ded6afbba98b4d14b8de806af3711e0f990ab155f897784512c66d4"
      sha256 cellar: :any_skip_relocation, x86_64_linux:   "eeb6d28feb23082eda967ee7d31a59ca755ecbe989776b7a96d7d12951ede757"
    end
  
    def install
      system "cargo", "install", *std_cargo_args
    end
  
    test do
        system "#{bin}/cargonode", "--version"
    end
end

