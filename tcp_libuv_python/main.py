import asyncio

class MyProtocol(asyncio.Protocol):
    def __init__(self):
        self.transport = None

    def connection_made(self, transport: asyncio.transports.Transport):
        self.transport = transport

    def data_received(self, data: bytes):
        number_of_commands = data.count(b"\n")
        self.transport.writelines([b"+PONG\r\n"] * number_of_commands)

def main():
    import uvloop
    asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())
    loop = asyncio.get_event_loop()
    coro = loop.create_server(MyProtocol, "127.0.0.1", 8888)
    server = loop.run_until_complete(coro)
    loop.run_forever()

main()
