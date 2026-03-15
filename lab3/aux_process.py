import sys
import socket

def main():
    mode = sys.argv[1]

    if mode == "pipe":
        data = sys.stdin.read().strip()
        num = int(data)
        print(f"{num * 2}")
        
    elif mode == "socket":
        port = int(sys.argv[2])
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect(("127.0.0.1", port))
        data = s.recv(1024).decode().strip()
        num = int(data)
        s.sendall(f"{num * 2}\n".encode())
        s.close()

    elif mode == "file":
        filepath = sys.argv[2]
        with open(filepath, "r") as f:
            data = f.read().strip()
        num = int(data)
        with open(filepath, "w") as f:
            f.write(f"{num * 2}")

if __name__ == "__main__":
    main()
