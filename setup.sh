git clone https://git.sovietlinux.ml/sovietlinux/CCCP.git -b rewrite
cd CCCP
sudo make libspm
cp bin/libspm.so /usr/lib/
cd ..
rm -rf CCCP