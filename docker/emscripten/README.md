set -e
set -u
set -o pipefail

cd /
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 2.0.7
./emsdk activate 2.0.7



# The node install script fetched and executed here will update the
# apt source list, hence the second apt-get update is necessary.
curl -s -S -L https://deb.nodesource.com/setup_14.x | bash -
apt-get update
apt-get install -y nodejs