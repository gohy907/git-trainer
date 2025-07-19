package main

type choice struct {
	title            string
	description      []string
	needConfifmation bool
}

type choiceType int

const (
	checkForCompletion choiceType = iota
	restartTask
)
