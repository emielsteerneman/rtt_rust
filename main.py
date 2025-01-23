import os
import socket

os.makedirs("python_protos", exist_ok=True)
os.system("protoc -I ./crates/protos/proto --python_out=python_protos ./crates/protos/proto/*.proto")
os.system("protol --create-package --in-place --python-out python_protos protoc --proto-path=crates/protos/proto ssl_vision_wrapper.proto")

import python_protos.ssl_vision_wrapper_pb2 as W

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

### Receive anything on port 22222
sock.bind(("127.0.0.1", 22222))
print("Waiting for response...")
while True:
    data, addr = sock.recvfrom(4096)
    print("Received data from", addr)
    wrapper = W.SSL_WrapperPacket()
    wrapper.ParseFromString(data)
    print(wrapper)

### How to send something
# sock.sendto(some_message.SerializeToString(), ("localhost", 8008))