package main

import (
	"bufio"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"os/exec"
	"strings"
	"sync"
)

const (
	listenAddr = "localhost:8999"
)

func main() {
	server := http.Server{
		Addr:    listenAddr,
		Handler: http.HandlerFunc(serveHTTP),
	}
	wg := sync.WaitGroup{}
	wg.Add(2)

	go func() {
		defer wg.Done()
		fmt.Println("Starting server at", listenAddr)
		err := server.ListenAndServe()
		if err != nil {
			fmt.Println("Server exit with error", err)
		}
	}()

	go func() {
		defer wg.Done()
		cmd := exec.Command("cloudflared", listenAddr)
		fmt.Println("Running command", cmd.String())
		stdErr, err := cmd.StderrPipe()
		if err != nil {
			fmt.Println("Failed to get std error pipe")
			return
		}
		go streamStdErr(stdErr)

		if err := cmd.Start(); err != nil {
			fmt.Println(cmd, "cloudflared started with error", err)
			return
		}

		if err := cmd.Wait(); err != nil {
			fmt.Println(cmd, "cloudflared exited with error", err)
		}
	}()

	wg.Wait()
	fmt.Println("Request inspector terminated")
}

func serveHTTP(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Request host: ", r.Host)
	fmt.Println("Requst headers: ", r.Header)
	body, err := ioutil.ReadAll(r.Body)
	if err != nil {
		fmt.Println("Failed to read request body, err: ", err)
		return
	}
	fmt.Println("Request body: ", string(body))
	w.Write([]byte("Read your request\n"))
}

func streamStdErr(stderr io.ReadCloser) {
	scanner := bufio.NewScanner(stderr)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		m := scanner.Text()
		// Print tunnel hostname
		if strings.Contains(m, "trycloudflare.com") {
			fmt.Println(m)
		}
	}
}
