package main

import (
	"net"
	"strings"
)

func handleRequest(conn net.Conn) {
	defer conn.Close()
	buf := make([]byte, 4096)
	for {
		var count int
		n, err := conn.Read(buf)
		if err != nil {
			break
		}
		for i := 0; i < n; i++ {
			if buf[i] == '\n' {
				count++
			}
		}
		conn.Write([]byte(strings.Repeat("+PONG\r\n", count)))
	}
}

func main() {
	listener, err := net.Listen("tcp", "127.0.0.1:8888")
	if err != nil {
		panic(err)
	}

	defer listener.Close()

	for {
		// skip errors
		conn, _ := listener.Accept()

		go handleRequest(conn)
	}
}
