class Cargonode < Formula
    desc "CLI for Node.js development workflows"
    homepage "https://github.com/xosnrdev/cargonode"
    url "https://github.com/xosnrdev/cargonode/archive/refs/tags/0.1.0.tar.gz"
    sha256 "42a936c2e863ebca38319c976dd229c38d0b5040862b14c6cf450b54ec7e5276"
    license "Apache-2.0"
    head "https://github.com/xosnrdev/cargonode.git", branch: "master"
  
    livecheck do
      url :stable
      strategy :github_latest
    end

    depends_on "rust" => :build
  
    bottle do
      sha256 cellar: :any,                 arm64_sonoma:   "ee67350dad5a14358be0dab0c494e6e7023ee2aa8a77d43457997650a8afc837"
      sha256 cellar: :any,                 arm64_ventura:  "ee67350dad5a14358be0dab0c494e6e7023ee2aa8a77d43457997650a8afc837"
      sha256 cellar: :any,                 arm64_monterey: "ee67350dad5a14358be0dab0c494e6e7023ee2aa8a77d43457997650a8afc837"
      sha256 cellar: :any,                 sonoma:         "e57e896300dcb33b4d64a99a227fdb288625dde11adaa73f91b9fbd866ee2be4"
      sha256 cellar: :any,                 ventura:        "e57e896300dcb33b4d64a99a227fdb288625dde11adaa73f91b9fbd866ee2be4"
      sha256 cellar: :any,                 monterey:       "e57e896300dcb33b4d64a99a227fdb288625dde11adaa73f91b9fbd866ee2be4"
      sha256 cellar: :any_skip_relocation, x86_64_linux:   "310995033f19ab7520815afe3cf847b1884a4d58f19c34e37c519d23ee759cb3"
    end
  
    def install
      system "cargo", "install", *std_cargo_args
    end
  
    test do
        system "#{bin}/cargonode", "--version"
    end
end
