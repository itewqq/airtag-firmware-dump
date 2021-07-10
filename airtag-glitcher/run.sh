make glitcher && echo "make success, starting socat on port 6666"
socat tcp-l:6666,fork exec:./glitcher,reuseaddr;