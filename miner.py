import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("eu1.ethermine.org", 4444))
sock.send(str.encode("sdfsdfds\n"))
print(sock.recv(4000))

##sock.send(str.encode("""{"params": ["0x931d5537aa741a62dadd03b33f74d9f45bcce70a", "password"], "id": 2, "method": "mining.authorize"}\n"""))
#print(sock.recv(4000))