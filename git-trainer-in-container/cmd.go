package main

import (
	"os"
	"os/exec"
	"strconv"
)

type Cmd struct {
	name string
	args []string
}

func newCmd(name string, args ...string) Cmd {
	return Cmd{name: name, args: args}
}

func runCmd(command Cmd) error {
	cmd := exec.Command(command.name, command.args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func enterTask(taskNumber int) {
	clear := newCmd("clear")
	run := newCmd("./run.sh", strconv.Itoa(taskNumber))
	runCmd(clear)
	runCmd(run)
	runCmd(clear)
}
