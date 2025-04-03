package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	// Retrieve the current username using the "USERNAME" environment variable.
	username := os.Getenv("USERNAME")
	if username == "" {
		username = "Unknown"
	}
	fmt.Printf("Current user: %s\n", username)

	// Retrieve the current working directory.
	cwd, err := os.Getwd()
	if err != nil {
		log.Fatalf("Error retrieving current directory: %v", err)
	}
	fmt.Printf("Current working directory: %s\n", cwd)

	// Retrieve the hostname using os.Hostname(), which is more portable.
	hostname, err := os.Hostname()
	if err != nil {
		log.Fatalf("Error retrieving hostname: %v", err)
	}
	fmt.Printf("Hostname: %s\n", hostname)

	// Prompt the user for an IP address or default to 8.8.8.8.
	fmt.Println("Enter an IP address to ping or press Enter to use the default (8.8.8.8):")
	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')
	if err != nil {
		log.Fatalf("Error reading user input: %v", err)
	}
	target := strings.TrimSpace(input)
	if target == "" {
		target = "8.8.8.8"
	}

	// Ping the target for 10 seconds.
	fmt.Printf("Pinging %s for 10 seconds...\n", target)
	cmd := exec.Command("cmd", "/c", "ping", target, "-n", "10")

	// Capture the output in real-time.
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		log.Fatalf("Error obtaining stdout pipe: %v", err)
	}

	if err := cmd.Start(); err != nil {
		log.Fatalf("Error starting ping command: %v", err)
	}

	scanner := bufio.NewScanner(stdout)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Printf("Error reading ping output: %v", err)
	}

	if err := cmd.Wait(); err != nil {
		log.Fatalf("Ping command finished with error: %v", err)
	}
}
